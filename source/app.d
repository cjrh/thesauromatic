import std.stdio;
import std.string;
import std.getopt;
import std.algorithm.setops;
import std.algorithm.iteration;
import std.algorithm.searching;
import std.algorithm.sorting;
import std.typecons;
import std.array;
import std.range;
import std.file: slurp, readText, read;
import std.conv;
import std.format;
import std.uni;

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
    auto f = File("source/mobylf.txt");
    auto wd = word ~ ",";
    foreach (line; f.byLine()) {
        if (line.startsWith(wd)) {
            string items = line.idup.findSplit(",")[2];
            return items.replace(",", "\n");
        }
    }
    return "";
}
