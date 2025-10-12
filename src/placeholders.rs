pub static EMPTY_SITE_CONFIG: &str = r#"# This is a template configuration file for a site uploaded with Statico.
# You NEED to do the following things to make this work:
# 1. Change the name of this file to `statico.toml`.
# 2. Fill out all the fields below.

# The alias of the server where the site is hosted, this must match one of the servers in the global config.
server = ""

# The name of the site, this will be used to refer to the site
name = ""

# The directory where the site's files are going to be stored. This is appended to the `base_directory` of the server settings
directory = ""

# The path from where to copy the files to the server.
output_dir = ""
"#;
