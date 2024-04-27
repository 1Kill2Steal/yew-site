# Python Utilities

This folder was meant to serve as a folder for all the Python utilities that are involved in this project.

Personal recommendation: Use [uv](https://github.com/astral-sh/uv) instead of vanilla pip.

## Set up instructions for pip

In this folder (`/py-utils/`) enter the following commands:

```bash
###############################################################################
# Installation
###############################################################################

# On macOS and Linux.
curl -LsSf https://astral.sh/uv/install.sh | sh

# On Windows.
powershell -c "irm https://astral.sh/uv/install.ps1 | iex"

# With pip.
pip install uv

# With pipx.
pipx install uv

# With Homebrew.
brew install uv

# With Pacman.
pacman -S uv

###############################################################################
# Virtual environment (in /py-utils/)
###############################################################################

uv venv

###############################################################################
# Final setting up (in /py-utils/)
###############################################################################

# On macOS and Linux.
source .venv/bin/activate

# On Windows.
.venv\Scripts\activate
```

## Required Python Utilities

1. [pysaucenao](https://github.com/rainyroads/pysaucenao)

- Purpose: [SauceNAO](https://saucenao.com/) is a website used for finding the
  image sources of artworks. It's the primary utility used for the
  `/hutao/json/artist_credits.json` file.

2. [python-dotenv](https://github.com/theskumar/python-dotenv)

- Purpose: Managing your environment variables (You need to set this up
  yourself).

**Short Setup Guide**

- Go on [SauceNAO](https://saucenao.com/)

- Go to `account` -> `api` -> copy the `api key` value.

- Make a `.env` file under this directory.

- Enter the following:

```env
SAUCENAO_API_KEY=your-copied-api-key
```

> [!NOTE]
> The required utilities can be installed via the following commands (Replace
> `flask` with the utilities from the required utilities list.) (There's also a
> `requirements.txt` file).

```bash
uv pip install flask                # Install Flask.
uv pip install -r requirements.txt  # Install from a requirements.txt file.
uv pip install -e .                 # Install the current project in editable mode.
uv pip install "package @ ."        # Install the current project from disk.
uv pip install "flask[dotenv]"      # Install Flask with "dotenv" extra.`
```
