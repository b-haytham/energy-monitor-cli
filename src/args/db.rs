use std::path::PathBuf;

use clap::Subcommand;


#[derive(Subcommand, Debug)]
#[clap(about = "Deals with mongodb database")]
pub enum DbCommands {
    #[clap(
        help = "Seed data to (storage collection)", 
        about ="Seed data to (storage collection)"
    )]
    Seed {
        /// mongodb uri
        #[clap(short, env = "MONGO_URI", help="mongodb uri")]
        uri: String,

        /// database name
        #[clap(short, env = "DATABASE_NAME", help="database name")]
        database: String,

        /// collection name
        #[clap(short, env = "COLLECTION_NAME")]
        collection: String,

        /// replica set name
        #[clap(short, env = "MONGO_REPL_SET")]
        repl_set: String,

        /// seed config: path to yaml file
        #[clap(short='p', parse(from_os_str))]
        seed_config: Option<PathBuf>
    },

    #[clap(
        help = "Drop all data from (storage collection)", 
        about = "Drop all  data from (storage collection)"
    )]
    Drop {
        /// mongodb uri
        #[clap(short, env = "MONGO_URI")]
        uri: String,

        /// database name
        #[clap(short, env = "DATABASE_NAME")]
        database: String,

        /// collection name
        #[clap(short, env = "COLLECTION_NAME")]
        collection: String,

        /// seed config: path to yaml file
        #[clap(short, env = "MONGO_REPL_SET")]
        repl_set: String,
    },
}
