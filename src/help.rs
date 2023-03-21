pub const HELP_TEXT: &str = "Usage: renvsubst [FLAGS] [FILTERS] [INPUT] [OUTPUT] | [-h | --help | --version]

renvsubst will substitute all (bash-like) environment variables in the format of $VAR_NAME, ${VAR_NAME} or ${VAR_NAME:-DEFAULT_VALUE} with their corresponding values from the environment or the default value if provided. If the variable is not valid, it remains as is.
A valid variable name starts with a letter or underscore, followed by any combination of letters, numbers, or underscores.

Flags:
  -u, --fail-on-unset              Fails if an environment variable is not set.
  -e, --fail-on-empty              Fails if an environment variable is empty.
  -f, --fail                       Alias for --fail-on-unset and --fail-on-empty.
  -U, --no-replace-unset           Does not replace variables that are not set in the environment.
  -E, --no-replace-empty           Does not replace variables that are set but empty in the environment.
  -N, --no-replace                 Alias for --no-replace-unset and --no-replace-empty.
  -x, --no-escape                  Disables escaping of variables with two dollar signs ($$).
  -b, --unbuffer-lines             Do not buffer lines before printing.
                                   Saves memory, but may impact performance.

When the same flag is provided multiple times, renvsubst will throw an error.

Filters:
  -p, --prefix[=PREFIX]...         Only replace variables with the specified prefix.
                                   Prefixes can be specified multiple times.
  -s, --suffix[=SUFFIX]...         Only replace variables with the specified suffix.
                                   Suffixes can be specified multiple times.
  -v, --variable[=VARIABLE]...     Specify the variables to replace. If not provided,
                                   all variables will be replaced.
                                   Variables can be specified multiple times.

The variables will be substituted according to the specified prefix, suffix, or variable name. If none of these options are provided, all variables will be substituted. When one or more options are specified, only variables that match the given prefix, suffix, or variable name will be replaced, while all others will remain unchanged.
If multiple identical prefixes, suffixes or variables are provided, only one copy of each will be used.

Input:
  -i, --input[=FILE]               Input file path. Use '-' to read from stdin.
                                   Defaults to stdin if omitted.

Output:
  -o, --output[=FILE]              Output file path. Use '-' to write to stdout.
                                   Defaults to stdout if omitted.

General:
-h, --help                         Show this help text.
    --version                      Show the version of the program.

Escaping:
To retain a variable's original value and prevent it from being substituted by an environment variable, add a second dollar sign ($). The second dollar sign will be removed during substitution. Only valid variables must be escaped.

";
