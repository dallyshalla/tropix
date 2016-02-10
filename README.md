# tropix
trade automator for cryptocurrency exchange

#install
### Building from source

##### Ubuntu 14.04, 15.04, 15.10

```bash

# install rust stable
curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh

# install stable and make it default
sudo multirust update stable
sudo multirust default stable

# download and build tropix
git clone https://github.com/dallyshalla/tropix
cd tropix
cargo run --bittrexcli
```

##### OSX with Homebrew

```bash
# install multirust
brew update
brew install multirust

# install nightly and make it default
multirust update stable && multirust default stable

# download and build tropix
git clone https://github.com/dallyshalla/tropix
cd tropix
cargo run --bittrexcli
```


