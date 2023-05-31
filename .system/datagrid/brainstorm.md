# DataGrid

## Goals

The goal of the datagrid is to have efficient and roughly dedouplicated file system that can be used to store and retrieve arbitrary data blobs.
In addition to allowing for a file system like interface. All of the above is done through the storage of blobs indexed by hash.

Under the assumption we can read and write key value storage efficiently, we can build a file system on top of it.

## File Abstraction

The file abstraction is an abstraction that allows for the storage of files by hash. The way it works is when queried for hash X, the server looks at the key X in the data blob storage and it returns a FileData structure. The FileData structure is a JSON object that describes metadata about the files aswell as where to find the chunks that make up the file.

The chunks in turn are stored in the data blob storage aswell, and are indexed by their hash. The chunks are stored in a binary format, and are not JSON.

## IPFS Compatability

The IPFS Protocol utalizes [multiformat/cid](https://github.com/multiformats/cid) to represent content identifiers. The datagrid uses the same format to represent content identifiers. This means that the datagrid can be used as a drop in replacement for IPFS, exchange files with IPFS servers, and use IPFS clients to interact with the datagrid.

## Ownership

Every bit of data is important. Especially when it comes to ranking of data relevance. The datagrid allows for users to _adopt ownership_ over a file. This means that so long as the users interest in this file stays the file stays relevant. When a datagrid client is operating it might perform maintenance migrating deserted files to for ex. colder storage.

## Filesystem abstraction

Just like we are able to store `File` objects, we are also able to store `Directory` objects. The `Directory` object is a JSON object that contains a list of `File` and `Directory` objects. This allows for the storage of a file system like structure. The `Directory` object is stored in the same way as the `File` object, by hash.
