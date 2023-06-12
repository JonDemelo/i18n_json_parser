# i18n_json_parser

This is a Rust-based JSON parser for i18n files translated using Google Translate.
It converts the naively translated text back into usable JSON for i18n services.

This is not intended to be a complete tool for all use cases and whatnot -- I'm merely hacking together something quick for my personal use case.

## Problem
The free Google Translate page translates the parameter names, so you can't just throw in the source language file and immediately use the produced translation file,
even if you accept that the translated elements aren't going to be perfect due to the complexities of language.

This is a problem using i18n features as you want the originating parameters with values of the new language, yet you can't tell Google Translate to only convert the values.

## Functionality
This takes a base file -- the originating lang file used in programming your app (typically English).
It cleans out all but that file's json param names and structure.
It then takes a source json file of a new language, produced by Google Translate output, and inserts its values into the appropriate slots of the original base language file.
Files most maintain original order, i.e. You cannot mess with positions after the translate process.

We then generate a new destination file with the original base-language JSON structure and param names, injected with the new language parameter values.

This will naively produce a translation file that can be used for i18n, via Google Translate.

## Disclaimer
This of course does not replace a manual translation service, however if one desires to rapidly generate a base set of translations to
a number of languages without necessarily waiting for complete and perfect translation, then this strategy is certainly "quick" and "free".

## Usage
Assumed file structure relative to main.rs:
`./src/translations/{targetBaseFile.json} <--- baseFile`

then any number of ...
`./src/translations/source/{targetSourceFile.json}`

Output will be the same filename of targets, yet in the dest folder.
`./src/translations/dest/{targetDestFile.json}`

Commandline is as follows:

`cargo build`

`cargo run baseFileName.json anyNumberOfLangSourceFiles.json anotherSourceFile.json againFile.json lastOne.json`

At minimum, must have one basefile argument, and one sourcefile argument, and will produce N dest files, where N is the number of source files.

## IMPORTANT NOTES ON JSON ERRORS FROM GOOGLE TRANSLATE
Source files must be valid json objects -- this is important to confirm as sometimes, depending on your translation output from Google Translate,
it may output an occasional few lines of invalid json.

Typically this comes in a form of a sentence punctuation being moved outside of a value's closing brackets, when translate
assumes the language defaults to punctuation outside of quotes.

e.g. 

`"original param": "original param value.",`

`"translated param": "translated param".,` <-- punctuation outside.

Also sometimes the seperating ": " will be missing from a line in the JSON output. Please make sure to fill in any missing seperators.

e.g. 

`"original param": "original param value",`

`"translated param translated param value",` <-- missing ": "

If you don't want these errors to be "naively" handled, i.e. just give you the file and you can clean it up in post, change this to false.
Otherwise, see fn clean_google_translate_errors

`const HANDLE_ERRORS: bool = true;`