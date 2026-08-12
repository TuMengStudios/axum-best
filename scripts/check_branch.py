#!/usr/bin/env python3
"""Block commits on protected branches (e.g. master/main).

Used as a pre-commit local hook via `.pre-commit-config.yaml`.
Run standalone with `--branch <name>` to test without switching branches.
"""

import argparse
import subprocess
import sys

# Branches that must not receive direct commits.
# Extend this list if more branches should be protected.
BLOCKED_BRANCHES = ("master", "main")


def current_branch() -> str:
    """Return the current git branch name (works even before the first commit)."""
    try:
        out = subprocess.run(
            ["git", "symbolic-ref", "--short", "-q", "HEAD"],
            check=True,
            capture_output=True,
            text=True,
        )
        return out.stdout.strip()
    except subprocess.CalledProcessError:
        # Not on a branch (e.g. detached HEAD): nothing to protect.
        return "HEAD"


def main() -> int:
    parser = argparse.ArgumentParser(description="Block commits on protected branches")
    parser.add_argument(
        "--branch",
        default=None,
        help="Branch name to check (default: current git branch)",
    )
    args = parser.parse_args()

    branch = args.branch or current_branch()

    if branch in BLOCKED_BRANCHES:
        print(
            f"check_branch.py: commits directly to '{branch}' are not allowed.\n"
            "  Create a feature branch first, e.g.:\n"
            "    git checkout -b feat/your-change\n"
            "  and open a pull request to merge your changes.",
            file=sys.stderr,
        )
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
