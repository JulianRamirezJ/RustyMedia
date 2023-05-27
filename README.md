# RustyMedia
### Teacher: Édison Valencia Díaz (evalencia@eafit.edu.co)
### - Julian Andres Ramirez Jimenez (jaramirezj@eafit.edu.co)
### - Daniel Arango Hoyos (darangoh@eafit.edu.co)

A proyect for develop a multimedia transfer tool via Tcp sockets.

## Compression & Decompression
To do the compression and decompression of archives we used the crate flate2, then compress the archives in a temporaly archive with a format .tar.gz in the client-side to send the information to the server-side, once the information reach the server, is allocated in a temporaly archive .tar.gz to proceed with the decompression of the content into a folder called "received".
