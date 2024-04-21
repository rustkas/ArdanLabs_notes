use authentication::{get_users, save_users, LoginRole, User};
use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all users.
    List,
    /// Add a user.
    Add {
        /// The user's login name
        username: String,

        /// The user's password (plaintext)
        password: String,

        /// Optional - mark as an admin
        #[arg(long)]
        admin: Option<bool>,
    },
    /// Delete user.
    Delete {
        /// The user's loging name to delete
        username: String,
    },
    /// Change a user's password
    ChangePassword {
        /// The user's loging name who's password change
        username: String,
        /// New password
        new_password: String,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Role");
    println!("{:-<40}", "");

    let users = get_users();
    users.iter().for_each(|(_, user)| {
        println!("{:<20}{:20?}", user.username, user.role);
    });
}

fn add_user(username: String, password: String, admin: bool) {
    let mut users = get_users();
    let role = if admin {
        LoginRole::Admin
    } else {
        LoginRole::User
    };
    let user = User::new(&username, &password, role);
    users.insert(username, user);
    save_users(users);
}

fn delete_user(username: String) {
    let mut users = get_users();
    if users.contains_key(&username) {
        users.remove(&username);
        save_users(users);
        println!("{username} has deleted")
    } else {
        println!("{username} does not exists")
    }
}

fn change_password(username: String, password: String) {
    let mut users = get_users();
    if let Some(user) = users.get_mut(&username) {
        user.password = authentication::hash_password(&password);

        println!(
            "{username} changes his/hers password to {new_password}",
            username = user.username,
            new_password = password
        );
    } else {
        println!("{username} does not exist");
    }
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_users();
        }
        Some(Commands::Add {
            username,
            password,
            admin,
        }) => {
            println!("Add user");
            add_user(username, password, admin.unwrap_or(false));
        }
        Some(Commands::Delete { username }) => {
            delete_user(username);
        }
        Some(Commands::ChangePassword {
            username,
            new_password,
        }) => {
            change_password(username, new_password);
        }
        None => {
            println!("No users her. Run with --help to see instructions.");
        }
    }
}
