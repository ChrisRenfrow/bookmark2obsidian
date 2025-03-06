use cli::Cli;

mod bookmarks;
mod cli;
mod obsidian;

fn main() {
    let args = Cli::start();
}

#[cfg(test)]
mod tests {
    use super::*;
}
