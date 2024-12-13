
import os
import sys

class Args:
    def __init__(self):
        self.script_path = os.path.dirname((os.path.dirname(os.path.abspath(sys.argv[0]))))
        self.vault_path = sys.argv[1]
        self.to_simplify = sys.argv[2]
        self.file_output_dir = os.path.join(self.vault_path, sys.argv[3])
        if not os.path.exists(self.file_output_dir):
            print(f"Output directory does not exist: {self.file_output_dir}")
            exit(1)

def transcribe(to_simplify, file_output_dir):
    cmd = f"cargo run --release --bin transcriber-bin -- --input {to_simplify} --output write --write-path {file_output_dir}"
    os.system(cmd)

def main():
    args = Args()
    os.chdir(args.script_path)
    transcribe(to_simplify=args.to_simplify, file_output_dir=args.file_output_dir)



if __name__ == "__main__":
    main()