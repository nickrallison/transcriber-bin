
import os
import sys

quotations = ["'", '"']
def trim_start_and_end(string: str, chars: list[str]) -> str:
    result = string
    for quote in quotations:
        if result.startswith(quote) and result.endswith(quote):
            result = result[1:-1]
    return result


class Args:

    def __init__(self):
        self.script_path = os.path.dirname((os.path.dirname(os.path.abspath(sys.argv[0]))))
        self.vault_path = sys.argv[1]
        self.to_simplify = trim_start_and_end(sys.argv[2], quotations)
        self.file_output_dir = os.path.join(self.vault_path, sys.argv[3])
        if not os.path.exists(self.file_output_dir):
            print(f"Output directory does not exist: {self.file_output_dir}")
            exit(1)

def transcribe(to_simplify, file_output_dir):
    build_cmd = f"cargo build --release"
    os.system(build_cmd)

    # windows sets ext to .exe, otherwise none
    if sys.platform == "win32":
        ext = ".exe"
    else:
        ext = ""

    run_cmd = f".\\target\\release\\transcriber-bin{ext} --input \"{to_simplify}\" --output write --write-path \"{file_output_dir}\""
    # print(run_cmd)
    os.system(run_cmd)

def main():
    args = Args()
    os.chdir(args.script_path)
    transcribe(to_simplify=args.to_simplify, file_output_dir=args.file_output_dir)



if __name__ == "__main__":
    main()