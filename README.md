
# mdbook Combiner
The utility aims to help solve the issue of an org having multiple mdbooks spread across repos but also want the option of producing a single mdbook for convenience and searchability of an entire orgs information.

# Examples
In general, the tool expects to have a folder that contains several SUMMARY.md or subfolders that contain them. I have used the repo tool to clone multiple repos to construct this folder but its documentation is err not great so your mileage may vary. A shell script that calls clone would probably work as well.

You can then call the tool as such ```mdbook-combiner -m METAFOLDER``` this will now output a new SUMMARY.md that can be fed into mdbook to create a combined mdbook.


## With unsorted markdown files

You can also use the -j flag to pass in folders containing markdown files, this will generate a SUMMARY for those files based on file location and add them into the combined output.
