use clap::Subcommand;


#[derive(Subcommand, Debug)]
#[clap(about = "Deals with mongodb database")]
pub enum DbCommands {
    #[clap(
        help = "Seed data to (storage collection)", 
        about ="Seed data to (storage collection)"
    )]
    Seed {
        #[clap(short, env = "MONGO_URI")]
        uri: String,

        #[clap(short, env = "DATABASE_NAME")]
        database: String,

        #[clap(short, env = "COLLECTION_NAME")]
        collection: String,

        #[clap(short, env = "MONGO_REPL_SET")]
        repl_set: String,
    },

    #[clap(
        help = "Drop all data from (storage collection)", 
        about = "Drop all  data from (storage collection)"
    )]
    Drop {
        #[clap(short, env = "MONGO_URI")]
        uri: String,

        #[clap(short, env = "DATABASE_NAME")]
        database: String,

        #[clap(short, env = "COLLECTION_NAME")]
        collection: String,

        #[clap(short, env = "MONGO_REPL_SET")]
        repl_set: String,
    },
}
