# RustyMedia
### Teacher: Édison Valencia Díaz (evalencia@eafit.edu.co)
### - Julian Andres Ramirez Jimenez (jaramirezj@eafit.edu.co)
### - Daniel Arango Hoyos (darangoh@eafit.edu.co)

This program was developed in Rust and consists of a client-server file transfer system. The client is responsible for sending files to the server, while the server receives and stores the files. The file transfer is performed by splitting the file into chunks and sending them over a TCP connection.

The client application takes two command-line arguments: the path to the file to be sent and the destination address of the server in the format `ip_address:port`. It reads the file, compresses it using Gzip compression, and sends it to the server. Once the file transfer is completed, the client terminates.

On the other hand, the server application listens for incoming connections from clients on a specified port. When a new client connects, it spawns a new thread to handle the client's request. This allows the server to handle multiple clients concurrently, utilizing the concept of multiprocessing.

Upon receiving a file from a client, the server decompresses the file using Gzip decompression and stores it on the server's file system. The server maintains the original file name and saves the received files in a designated output folder.

By using Gzip compression, the program achieves efficient file transfer by reducing the size of the transmitted data. This helps optimize network bandwidth utilization and minimize transfer times.

Overall, this file transfer system showcases key concepts in Rust programming, such as concurrent processing with threads, TCP socket communication, and file compression using Gzip.
![FinalProyectOS](https://github.com/JulianRamirezJ/RustyMedia/assets/57159295/291e215e-8659-47ee-a072-381a9cfa9f8e)

## Compression & Decompression
To do the compression and decompression of archives we used the crate flate2, then compress the archives in a temporaly archive with a format .tar.gz in the client-side to send the information to the server-side, once the information reach the server, is allocated in a temporaly archive .tar.gz to proceed with the decompression of the content into a folder called "received".
