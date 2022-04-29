/* NOTES
All files go into blobs with hash as name
Commits have maps of file names to hashes

Blob refers to files in the blobs dir with contents consisting of committed files, with file
names representing their hash
It also refers to this struct, which represents a mapping between these files and file names. It
should be embedded in commits and the staging area.
 */

// We are going to need to do a lot of indexing and searches of blobs
// Is there any way to use language features to make this easier? Would we be better served by
// a collection?
struct Blob {
    hash: String,
    name: String,
}
