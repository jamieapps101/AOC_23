# default sourceme to setup pre-commit checks

PRE_EXISTING="1"
if [ ! -d .venv ]; then
    PRE_EXISTING="0"
    python3 -m venv .venv
fi

source .venv/bin/activate

if [ "$PRE_EXISTING" -eq "0" ]; then
    python3 -m pip install -r requirements.txt
    pre-commit install -c .pre-commit-config.yaml
fi
