"""
NOTE: You need to generate your API key
Make a saucenao account -> go to account -> go to api
The file needs to have this structure:

||| start of file (.env)

SAUCENAO_API_KEY=your-key

||| end of file

"""

from dotenv import load_dotenv
from os import getenv, path
from pathlib import Path
from pysaucenao import SauceNao

import argparse
import re
import asyncio

def set_regex_number(item_to_match: str):
    """
    Match for a number at either the end (higher priority) or the start (lower
    priority) of a file name.
    """
    regexpr = r"(_)(\d+)(.)|(\d+)(_)"
    match = re.search(regexpr, item_to_match)
    if match:
        target = match.group(2)
        if not match.group(2):
            target = match.group(4)
        return int(target)
    else:
        return None

async def main():
    """
    Handle all the API requests from SauceNAO. You can pass an argument to
    remove the rate limits.

    NOTE: If you don't have a premium account then you shouldn't remove the
    rate limits because otherwise the script will crash after the 4th request.
    """

    # Arguments parsing
    parser = argparse.ArgumentParser(description='Description of your program.')
    parser.add_argument('--rate-limit', '-rl', \
                        type=bool, default=True, \
                        help='The SauceNAO API Rate Limit toggle (On by default)'\
                        )

    # It doesn't HAVE to be hard coded but it's easier to hard code it in this use case.
    parser.add_argument('--skip-existing', '-se', \
                        type=bool, default=True, \
                        help='Skip the hardcoded existing image credits (On by default)' \
                        )

    args = parser.parse_args()

    rate_limit = args.rate_limit
    skip_existing = args.skip_existing

    # Path variables
    project_root = Path(path.dirname(path.dirname(path.abspath(__file__))))
    pics_path = Path(path.join(project_root, "hutao/pics_uncompressed/"))

    # Managing environment variables
    load_dotenv()
    SAUCENAO_API_KEY = getenv("SAUCENAO_API_KEY")
    if not SAUCENAO_API_KEY:
        print("Please set your SAUCENAO_API_KEY environment variable in the",
        ".env file.")
        return

    sauce = SauceNao(api_key=SAUCENAO_API_KEY)

    # Initialize the rate limit management
    rate_limit_counter = 0

    # Set your set with already existing items (so you can skip them)
    #
    # Reason: SauceNAO has a 100 requests per day limit so make sure you don't
    # use unneeded requests!
    existing_credits = {101, 102, 103, 107, 108, 113, 116, 118, 120, 121, 126,
                        128, 131, 132, 133, 134, 135, 136, 137, 14, 142, 144,
                        145, 146, 151, 17, 18, 21, 22, 25, 31, 43, 44, 45, 50,
                        6, 61, 67, 69, 8, 81, 90, 96, 98, 72, 16, 35, 24, 52,
                        79, 109}

    ###################
    # JSON generation #
    ###################

    print("{")

    for file in pics_path.iterdir():
        if not file.is_file():
            continue

        # Call the img_id from the regex function.
        img_id = set_regex_number(item_to_match=file.name)

        # Skip already checked values
        if skip_existing and existing_credits.__contains__(img_id):
            continue

        results = await sauce.from_file(file.__fspath__())

        # Rate limits are 4 requests per 30 seconds.
        if rate_limit:
            rate_limit_counter += 1

        if rate_limit_counter == 4:
            rate_limit_counter = 0
            await asyncio.sleep(30)

        # Print out the JSON values pairs.
        if results and results[0].similarity > 80:
            print(f"  \"{img_id}\": \"{results[0].source_url} {results[0].author_name}\"")
        else:
            print(f"  \"{img_id}\": \"\"")

    print("}")

if __name__ == "__main__":
    asyncio.run(main())
