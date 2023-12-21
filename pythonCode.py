import socket

def send_udp_request(message, server_ip, server_port):
    # Create a UDP socket
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as s:
        # Send the message to the server
        s.sendto(message.encode(), (server_ip, server_port))
    
        # Set a timeout for receiving the response (in seconds)
        s.settimeout(5)

        try:
            # Receive the response from the server
            data, server_address = s.recvfrom(1024)
            print(f"Received response from {server_address}: {data.decode()}")
        except socket.timeout:
            print("No response received within the timeout period.")

if __name__ == "__main__":
    # Replace these values with the appropriate server IP and port
    server_ip = "127.0.0.1"
    server_port = 8080

    # Message to send
    message = "Hello, UDP Server!"

    # Send UDP request
    send_udp_request(message, server_ip, server_port)