#!/bin/bash
git checkout master &&
git pull &&
git checkout space_efficient &&
git merge --no-edit master &&
git push &&
git checkout fnv_hash &&
git merge --no-edit master &&
git push &&
git checkout fnv_build &&
git merge --no-edit master &&
git push &&
git checkout rustc_hash_hash &&
git merge --no-edit master &&
git push &&
git checkout rustc_hash_build &&
git merge --no-edit master &&
git push &&
git checkout hash_brown_hash &&
git merge --no-edit master &&
git push &&
git checkout hash_brown_build &&
git merge --no-edit master &&
git push 
git checkout space_effcient_128 &&
git merge --no-edit master &&
git push 
git checkout space_effcient_max &&
git merge --no-edit master &&
git push 