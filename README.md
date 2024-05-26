# Hay Ala Salah iA Scraper

A GPT-4o powered prayer time scraper

## How it works
It takes a screenshot of a mosque webpage (after optionally scrolling and/or sleeping) and then passes the screenshot image to the ChatGPT vision API to extract the data

## Pre-reqs
1. Firefox should be installed
2. Install geckodriver (```cargo install geckodriver``` or google for the release binaries)

## Usage
```bash

cargo build
export OPENAI_API_KEY=<your OpenAI key>
cd target/debug # or target/release
./fetcher <URL to screenshot> [--scroll <pixels to scroll>] [--sleep <time to sleep before screenshotting>] | ./interpreter

```
