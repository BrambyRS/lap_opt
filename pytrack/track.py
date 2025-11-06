import struct

MAX_NODES = 65535 # Maximum number of nodes that can be stored in a .trk file

class Track:
    def __init__(self, name: str, nodes: list[tuple[float, float]]):
        self.name = name
        self.nodes = nodes

    def write_trk(self, filename: str) -> None:
        if len(self.nodes) > MAX_NODES:
            raise ValueError(f"Number of nodes exceeds maximum of {MAX_NODES}")

        with open(filename, 'wb') as f:
            # Write 128-byte header
            header = bytearray(128)
            header[0:4] = b'TRKF'  # Magic number
            header[4:8] = (0).to_bytes(4, byteorder='little') # Major version
            header[8:12] = (1).to_bytes(4, byteorder='little') # Minor version
            # Add the track name (up to 64 bytes)
            name_bytes = self.name.encode('utf-8')[:64]
            header[12:12+len(name_bytes)] = name_bytes
            f.write(header)
            # Write number of nodes as unsigned 16-bit integer
            f.write(len(self.nodes).to_bytes(2, byteorder='little', signed=False))
            # Write each node as two 32-bit floats (x, y)
            for x, y in self.nodes:
                f.write(struct.pack('<ff', x, y))
