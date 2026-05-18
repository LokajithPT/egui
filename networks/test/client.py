import socket
import sys

def main():
    # 1. Validate command line arguments
    if len(sys.argv) < 2:
        print("Usage: python client.py <command>")
        print("Available commands: on, off, vers")
        sys.exit(1)

    # 2. Extract the argument (on, off, or vers)
    command = sys.argv[1]

    # Target server configuration
    host = "127.0.0.1"
    port = 8080

    try:
        # 3. Create a raw TCP socket
        # AF_INET = IPv4, SOCK_STREAM = TCP
        client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

        # 4. Connect to the Rust server
        client_socket.connect((host, port))

        # 5. Send data. Strings must be encoded to raw bytes via .encode()
        payload = command.encode('utf-8')        client_socket.sendall(payload)

        # 6. Receive the raw response buffer (up to 1024 bytes)
        response_bytes = client_socket.recv(1024)
        
        # 7. Decode raw bytes back to a readable string
        response_text = response_bytes.decode('utf-8')
        print(f"Server response: {response_text.strip()}")

    except ConnectionRefusedError:
        print(f"Error: Could not connect to the server at {host}:{port}. Is the Rust server running?")
    except Exception as e:
        print(f"An error occurred: {e}")
    finally:
        # 8. Always close the socket resource cleanly
        client_socket.close()

if __name__ == "__main__":
    main()
