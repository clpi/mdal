# Markdown Analysis LSP

---

## About 

### The Markdown file format
- The **markdown** file format `*{md, mdx}` is, I'm sure, no use explaining to you, if you're here. The goal of the project is to support markdown files, native according to CommonMark specs *first and foremost*. We are building our system primarily around figuring out how one can semantically construct and generate complex data structures which produce valuable insight, without having to change the medium at all, or change the way people already do things. If you subscribe to a certain method of using Markdown to jot information down -- our goal is not to have your habits adapt to the LSP and ecosystem as a whole, but the other way around.

### The Idlex file format
- The **idlex** file format `*.{mdi, idx, id}` is a superset of markdown intended to accomodate the larger data-in-text goal of MDA without requiring anything of the uncommitted user. Normal markdown will be accepted by in large, while a host of new methods to explicitly and implicitly demarcate asssociations and relationships between entities and elements in your natural language input are provided.


## The MDA Graph
- The key to the puzzle of semantically organizing one's own otherwise unorganized data is persistent storage that is efficient, practical, and allows for multi-dimensional expansion without too much effort. Using the MDA language server will automatically establish such a store on the machine it's run on, with utilities built-in to sync between machinse and devices. 


### Differences from CommonMark
1) Explicit data tags are denoted with `:datatag:` (i.e. surrounding the datatag with colons) or alternatively `#datatag ` (i.e. pound prefixed with trailing whitespace)

1) Datablocks may be defined across any block of code, spanning characters in a line or lines in a file, or files in a workspace. `@block ... :` denotes the beginning of datablock `block` with arguments/associative parameters `....`, and the end of a block is denoted `:@`. Different blocks may be defind and qualified by user-input, such that blocks such as `++block ... = <data> ;` for example, might be defind.

3) Builtin primtive datatypes, with boolean, integer, float, date, etc. values are parsed from natural language data, or data explicitly passed (and possibly associated/tagged/linked to other explicit/implciit data) and are stored in the MDA database.
### Goals

### Non-goals

### Links

