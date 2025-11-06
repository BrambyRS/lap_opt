import os

from track import Track

def generate_straight(length: float) -> Track:
    # Generate a straight track with the given length
    name = f"{length:.0f} m Straight"
    track = Track(name, [(0, 0), (length, 0)])
    return track

def main():
    pytrack_dir = os.path.dirname(os.path.abspath(__file__))

    track = generate_straight(100)
    track.write_trk(os.path.join(os.path.dirname(pytrack_dir), "tracks", "straight.trk"))

if __name__ == "__main__":
    main()
