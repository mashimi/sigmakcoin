// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract AITaskMarketplace {
    struct TrainingTask {
        address creator;
        string modelCid;
        string datasetCid;
        uint256 reward;
        uint256 deadline;
        bool completed;
        address winner;
        string resultCid;
    }
    
    struct Miner {
        address minerAddress;
        uint256 totalRewards;
        uint256 reputation;
        uint256 tasksCompleted;
    }
    
    mapping(uint256 => TrainingTask) public tasks;
    mapping(address => Miner) public miners;
    mapping(uint256 => mapping(address => string)) public submissions;
    
    uint256 public taskCounter;
    uint256 public minStake = 0.1 ether;
    
    event TaskCreated(uint256 taskId, address creator, uint256 reward);
    event ProofSubmitted(uint256 taskId, address miner, string proofHash);
    event TaskCompleted(uint256 taskId, address miner, string resultCid);
    
    // Create a new AI training task
    function createTask(
        string memory _modelCid,
        string memory _datasetCid,
        uint256 _reward,
        uint256 _durationHours
    ) external payable {
        require(msg.value >= _reward, "Insufficient reward");
        require(_reward > 0, "Reward must be positive");
        
        taskCounter++;
        tasks[taskCounter] = TrainingTask({
            creator: msg.sender,
            modelCid: _modelCid,
            datasetCid: _datasetCid,
            reward: _reward,
            deadline: block.timestamp + (_durationHours * 1 hours),
            completed: false,
            winner: address(0),
            resultCid: ""
        });
        
        emit TaskCreated(taskCounter, msg.sender, _reward);
    }
    
    // Miner submits a proof of training
    function submitProof(uint256 _taskId, string memory _proofHash, string memory _resultCid) external {
        TrainingTask storage task = tasks[_taskId];
        require(!task.completed, "Task already completed");
        require(block.timestamp < task.deadline, "Task deadline passed");
        
        submissions[_taskId][msg.sender] = _proofHash;
        
        emit ProofSubmitted(_taskId, msg.sender, _proofHash);
    }
    
    // Validator approves a miner's work
    function approveWork(uint256 _taskId, address _miner) external {
        TrainingTask storage task = tasks[_taskId];
        require(!task.completed, "Task already completed");
        require(msg.sender != task.creator, "Only validators can approve");
        require(bytes(submissions[_taskId][_miner]).length > 0, "No proof submitted");
        
        task.completed = true;
        task.winner = _miner;
        task.resultCid = submissions[_taskId][_miner];
        
        // Pay the miner
        payable(_miner).transfer(task.reward);
        
        // Update miner stats
        miners[_miner].totalRewards += task.reward;
        miners[_miner].tasksCompleted++;
        miners[_miner].reputation += 10;
        
        emit TaskCompleted(_taskId, _miner, submissions[_taskId][_miner]);
    }
    
    // Stake to become a validator (min 0.1 ETH)
    function stake() external payable {
        require(msg.value >= minStake, "Minimum stake not met");
        miners[msg.sender].reputation += 1;
    }
    
    // Get miner stats
    function getMinerInfo(address _miner) external view returns (uint256, uint256, uint256) {
        return (
            miners[_miner].totalRewards,
            miners[_miner].reputation,
            miners[_miner].tasksCompleted
        );
    }
}
