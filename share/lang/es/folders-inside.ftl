
# machine translated (Spanish), not yet reviewed by a person.

# Red notice at the foot of Folders when something is taken out of an archive and nothing in it is picked.
folders-inside-pick-first = Selecciona primero algo del archivo comprimido.

# Red notice at the foot of Folders when things are taken out of an archive into a folder that is on another computer.
folders-inside-only-here = Solo se pueden extraer cosas de un archivo comprimido a una carpeta de este equipo.

# Red notice at the foot of Folders when an archive cannot be read: its name, then the reason (a short phrase).
folders-inside-unread = Folders no pudo leer { $name }: { $why }.

# Red notice at the foot of Folders when some things taken out of an archive were refused because their names reach outside the folder; the number refused.
folders-inside-refused =
    { $n ->
        [one] Folders no pudo extraer 1 elemento, porque su nombre apunta fuera de la carpeta o hay algo en medio.
       *[other] Folders no pudo extraer { $n } elementos, porque sus nombres apuntan fuera de la carpeta o hay algo en medio.
    }

# Red notice at the foot of Folders when taking something out of an archive failed; the reason.
folders-inside-failed = Folders no pudo extraerlo del archivo comprimido: { $why }.

