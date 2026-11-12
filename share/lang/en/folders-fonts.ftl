# Words of Folders showing and installing fonts.

## Fonts (src/files/fonts.rs)

# Reason shown in the viewer in place of a font sample when the font file is too large to read.
folders-fonts-too-large = it's too large to show
# Reason shown in the viewer in place of a font sample when the file is not a font Folders can read.
folders-fonts-unreadable = it isn't a font Folders can read
# Red note when the person installs a font they already have.
folders-fonts-have-it = This font is already installed.
# Red note when the fonts folder could not be made; the reason follows.
folders-fonts-install-failed = Folders couldn't install the font: { $why }.
# Name of the fonts being installed, in the progress and done words, when more than one font is installed.
folders-fonts-many =
    { $n ->
        [one] 1 font
       *[other] { $n } fonts
    }
# Progress words while fonts are being installed; the name or count of the fonts.
folders-fonts-doing = Installing { $what }
# Note when fonts have been installed; the name or count of the fonts.
folders-fonts-done = Installed { $what } for apps you open from now on.
# Word for what happened to fonts that could not be done, in the list of failures.
folders-fonts-verb = installed
# Tip of the undo button after fonts were installed; the name or count of the fonts.
folders-fonts-undo = Undo installing { $what }
