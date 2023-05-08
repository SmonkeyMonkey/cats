This crate implementing unsupervised machine learning model.
In this project we building a model that can distinguish different cat breeds.The model collected body size measurements from three cat breeds: Persian, British Shorthair, and Ragdoll . Since these three breeds have slightly different body heights and lengths, we  run a K-means clustering model on these two features.
In config/generate.toml sets body heights and lengths 

## Optional
if you change confing/generate.toml and generate new csv file(remove old csv files) parametrs

For generating data to CSV file run:

```bash
cargo run --bin generate -- -c config/generate.toml > training_data.csv 
```
remove "> training_data.csv" for print to stdout in csv format
For train your model :
```bash
cat training_data.csv | cargo run --bin cluster > results.csv
```

## Installation
Clone repository
```bash
git clone https://github.com/SmonkeyMonkey/cats.git
```

For visualisattion required install gnuplot:
Debian based: 
```bash
sudo apt-get install gnuplot 
```

Arch: 
```bash
sudo pacman -S gnuplot
``` 

CentOS / RedHat:
```bash
sudo yum install gnuplot
```

Fedora:
```bash
sudo dns install gnuplot
``` 

macOS:

   Using Homebrew: 

```bash
   brew install gnuplot
```
   Using MacPorts:
```bash
sudo port install gnuplot
```
## Usage
After installation gnuplot you can visualisation your data:

```bash
    cat results.csv | cargo run --bin plot 
```
