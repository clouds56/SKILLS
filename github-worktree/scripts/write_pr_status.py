#!/usr/bin/env python3

import argparse
import json
import subprocess
import sys
from pathlib import Path


def run_json(cmd):
    result = subprocess.run(cmd, capture_output=True, text=True, check=True)
    return json.loads(result.stdout)


def run_text(cmd):
    result = subprocess.run(cmd, capture_output=True, text=True, check=True)
    return result.stdout.strip()


def discover_branch():
    return run_text(["git", "branch", "--show-current"])


def discover_repo():
    data = run_json(["gh", "repo", "view", "--json", "nameWithOwner,defaultBranchRef"])
    default_branch = data["defaultBranchRef"]["name"]
    return data["nameWithOwner"], default_branch


def discover_prs(branch):
    return run_json(
        [
            "gh",
            "pr",
            "list",
            "--head",
            branch,
            "--state",
            "all",
            "--json",
            "number,state,title,url,headRefName,baseRefName,isDraft",
        ]
    )


def normalize_pr_status(prs):
    if not prs:
        return "none"
    if len(prs) == 1:
        return prs[0]["state"].lower()
    return "multiple"


def write_status(output_dir, payload):
    output_dir.mkdir(parents=True, exist_ok=True)
    (output_dir / ".gitignore").write_text("*\n", encoding="utf-8")
    status_path = output_dir / "status.json"
    status_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return status_path


def parse_args():
    parser = argparse.ArgumentParser(
        description="Write .github/_pr_/status.json from the current git/gh repository context."
    )
    parser.add_argument(
        "--output-dir",
        default=".github/_pr_",
        help="Directory that will receive .gitignore and status.json.",
    )
    parser.add_argument("--branch", help="Override the current branch name.")
    parser.add_argument("--repo-name", help="Override the GitHub repo nameWithOwner.")
    parser.add_argument("--default-branch", help="Override the repository default branch.")
    parser.add_argument(
        "--pr-status",
        choices=["none", "open", "closed", "merged", "multiple"],
        help="Override the normalized PR status.",
    )
    parser.add_argument(
        "--prs-json",
        help="Override the raw PR list with a JSON array string shaped like gh pr list --json output.",
    )
    return parser.parse_args()


def main():
    args = parse_args()

    branch = args.branch or discover_branch()
    if args.repo_name and args.default_branch:
        repo_name, default_branch = args.repo_name, args.default_branch
    else:
        repo_name, default_branch = discover_repo()

    if (args.repo_name and not args.default_branch) or (args.default_branch and not args.repo_name):
        print("Both --repo-name and --default-branch must be provided together.", file=sys.stderr)
        return 2

    prs = json.loads(args.prs_json) if args.prs_json else discover_prs(branch)
    pr_status = args.pr_status or normalize_pr_status(prs)

    payload = {
        "branch": branch,
        "default_branch": default_branch,
        "pr_status": pr_status,
        "prs": prs,
        "repo_name": repo_name,
    }
    status_path = write_status(Path(args.output_dir), payload)
    print(status_path)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
