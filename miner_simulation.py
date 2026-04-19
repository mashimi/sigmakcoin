import hashlib
import json
import time
import urllib.request

import torch
from transformers import AutoModelForCausalLM, AutoTokenizer
from datasets import load_dataset
from torch.optim import AdamW

VALIDATOR_URL = "http://localhost:3000/submit"


def sha256_hex(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def compute_transaction_hash(transaction: dict) -> str:
    payload = f"{transaction['sender']}|{transaction['recipient']}|{transaction['amount']}|{transaction['data_hash']}".encode()
    return sha256_hex(payload)


def compute_merkle_root(transactions):
    if not transactions:
        return "0" * 64
    hashes = [compute_transaction_hash(tx).encode() for tx in transactions]
    while len(hashes) > 1:
        next_hashes = []
        for i in range(0, len(hashes), 2):
            left = hashes[i]
            right = hashes[i + 1] if i + 1 < len(hashes) else left
            next_hashes.append(hashlib.sha256(left + right).hexdigest().encode())
        hashes = next_hashes
    return hashes[0].decode()


def build_task_package():
    return {
        "task_id": "poiw-001",
        "model_hash": "abcd1234efgh5678abcd1234efgh5678abcd1234efgh5678abcd1234efgh5678",
        "required_loss_drop": 0.01,
        "data_hash": "feedfacecafebeefdeadbeefcafefeedfacecafebeefdeadbeefcafefeedface",
    }


def build_header(parent_hash, transactions, task_package):
    return {
        "parent_hash": parent_hash,
        "merkle_root": compute_merkle_root(transactions),
        "task_id": task_package["task_id"],
        "model_hash": task_package["model_hash"],
        "timestamp": int(time.time()),
        "nonce": int(time.time() * 1000) & 0xFFFFFFFF,
    }


def submit_candidate(candidate):
    body = json.dumps(candidate).encode("utf-8")
    request = urllib.request.Request(
        VALIDATOR_URL,
        data=body,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    with urllib.request.urlopen(request, timeout=10) as response:
        return response.read().decode("utf-8")


def simulate_mining():
    print("Loading model...")
    model = AutoModelForCausalLM.from_pretrained(
        "TinyLlama/TinyLlama-1.1B-Chat-v1.0",
        torch_dtype="auto"
    )
    tokenizer = AutoTokenizer.from_pretrained("TinyLlama/TinyLlama-1.1B-Chat-v1.0")

    print("Loading dataset...")
    dataset = load_dataset("c4", "en", split="train", streaming=True)
    batch = [next(iter(dataset))["text"] for _ in range(8)]
    inputs = tokenizer(batch, return_tensors="pt", padding=True, truncation=True, max_length=128).to(model.device)

    with torch.no_grad():
        loss_before = model(**inputs, labels=inputs["input_ids"]).loss.item()
    print(f"Loss before: {loss_before:.4f}")

    optimizer = AdamW(model.parameters(), lr=1e-5)
    model.train()
    print("Running mobile AI work...")
    for step in range(20):
        outputs = model(**inputs, labels=inputs["input_ids"])
        loss = outputs.loss
        loss.backward()
        optimizer.step()
        optimizer.zero_grad()
        if step % 5 == 0:
            print(f" step {step}, loss: {loss.item():.4f}")

    with torch.no_grad():
        loss_after = model(**inputs, labels=inputs["input_ids"]).loss.item()
    print(f"Loss after: {loss_after:.4f}")
    print(f"Computed proof improvement: {loss_before - loss_after:.4f}")

    task_package = build_task_package()
    tx = {
        "sender": "phone-miner-1",
        "recipient": "ai-protocol",
        "amount": 1,
        "data_hash": task_package["data_hash"],
    }
    header = build_header("0" * 64, [tx], task_package)
    candidate = {
        "header": header,
        "transactions": [tx],
        "proof": {
            "loss_before": loss_before,
            "loss_after": loss_after,
        },
    }

    print("Submitting candidate block to validator...")
    result = submit_candidate(candidate)
    print(result)


if __name__ == "__main__":
    simulate_mining()