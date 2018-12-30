import std.stdio;
import std.string;
import std.getopt;
import std.algorithm.setops;
import std.algorithm.iteration;
import std.algorithm.searching;
import std.typecons;
import std.array;
import std.range;
import std.file: slurp;
import std.conv;
import std.format;

private immutable string mobylf = import("mobylf.txt");
static immutable string[string] data;

static this() {
    Tuple!(string, string, string) r;
    foreach (line; mobylf.lineSplitter()) {
        r = line.findSplit(",");
        data[r[0]] = r[2];
    }
}

void main(string[] args)
{
    auto helpInformation = getopt(args);
    if (helpInformation.helpWanted)
    {
        defaultGetoptPrinter(
                "Usage: thesauromatic <word>\n\n" ~
                "Given <word> this program will print out a list\n" ~
                "of synonyms, one per line. This makes it easy to\n" ~
                "consume with other tools.\n",
                helpInformation.options);
        return;
    }

    if (args.length >= 2) {
        writeln(syns(args[1]));
    } else {
        writeln("ERROR: At least one word must be specified.");
        writeln(syns("blase"));
    }
}

private string syns(const string word) {
    if ((word in data) is null)
        return "";
    return data[word].replace(",", "\n");
}
