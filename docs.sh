#!/bin/bash
cd movie_example
rm -rf target/doc
cargo doc
echo "<head><meta http-equiv='refresh' content='0; URL=https://movie.pzmarzly.pl/movie'></head>" > target/doc/index.html
rsync -aP --rsh=ssh target/doc/ wwwserver:~/websites/movie
cd ..
