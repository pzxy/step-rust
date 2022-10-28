set -e
if [ ! -d ./pcre2-10.40 ]; then
  unzip pcre2-10.40.zip
fi
pwd=$(pwd)
cd ./pcre2-10.40 &&
./configure --prefix="${pwd}"/pcre &&
make && make install &&

