# Words of the longer Folders jobs (packing, unpacking, wiping, picture work) as they appear in the lists of what failed.

## Jobs (src/files/jobs.rs)

# Line in a list of files a job could not do: the file's name, then in brackets why not.
folders-jobs-failed-why = { $name } ({ $why })
# Why a wipe left a file alone, after its name in the failed list: it has other names (hard links).
folders-jobs-refused-names = it's also saved under other names
# Why a wipe left something alone, after its name in the failed list: it is not a plain file.
folders-jobs-refused-not-file = it isn't a file
# Why a wipe left a file alone, after its name in the failed list: it changed while the wipe ran.
folders-jobs-refused-changed = it changed during the wipe
# Why a wipe left a file alone, after its name in the failed list: the disk refused.
folders-jobs-refused-disk = Folders couldn't wipe it
