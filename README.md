This crate implementing unsupervised machine learning model
In this project we building a model that can distinguish different cat breeds.The model collected body size measurements from three cat breeds: Persian, British Shorthair, and Ragdoll . Since these three breeds have slightly different body heights and lengths, we  run a K-means clustering model on these two features.
In config/generate.toml sets body heights and lengths 

Optional (if you change confing/generate.toml parametrs):
if you generate new csv file, remove results.csv
For generating data to CSV file run:
cargo run --bin generate -- -c config/generate.toml > training_data.csv // remove "> training_data.csv" for print to stdout in csv format
For train your model :
cat training_data.csv | cargo run --bin cluster > results.csv

For visualisattion required install gnuplot:
Debian based: 
    sudo apt-get install gnuplot 
Arch: 
    sudo pacman -S gnuplot
CentOS / RedHat:
    sudo yum install gnuplot
Fedora:
    sudo dns install gnuplot
macOS:
    Using Homebrew: 
        brew install gnuplot
    Using MacPorts:
        sudo port install gnuplot

After installation gnuplot you can visualisation your data:
    cat results.csv | cargo run --bin plot 
