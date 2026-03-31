import json
import os

def main():
    file_path = 'processed/human_eval_verus.jsonl'
    if not os.path.exists(file_path):
        print(f"Error: {file_path} not found.")
        return

    with open(file_path, 'r') as f:
        for line in f:
            if not line.strip():
                continue
            try:
                data = json.loads(line)
                if data.get("has_verus_impl") and len(data.get("impl_sig")) > 1:
                    print(data.get("task_id"))
                    for s in data.get("impl_sig"):
                        print(s)
                    print()
            except json.JSONDecodeError:
                continue

if __name__ == "__main__":
    main()
