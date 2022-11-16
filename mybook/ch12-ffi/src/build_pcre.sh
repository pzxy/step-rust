set -e
if [ ! -d ./pcre2-10.40 ]; then
  unzip pcre2-10.40.zip
fi

pwd=$(pwd)
cd ./pcre2-10.40 && ./configure --prefix="${pwd}"/pcre && make && make install

# pcre的unit width配置
file_name="../pcre/include/pcre2.h"
context="#define PCRE2_CODE_UNIT_WIDTH 8"
if [ $(uname) = "Darwin" ]; then
  sed -i '' '1 i\
'"${context}"'
' ${file_name}
else
  sed -i '1i '"${context}"'
' ${file_name}
fi



