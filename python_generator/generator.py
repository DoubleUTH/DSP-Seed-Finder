import math

class Vector3:
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z

    def __repr__(self):
        return f"Vector3({self.x}, {self.y}, {self.z})"

    @staticmethod
    def zero():
        return Vector3(0, 0, 0)

    def distance_sq_from(self, other):
        dx = self.x - other.x
        dy = self.y - other.y
        dz = self.z - other.z
        return dx * dx + dy * dy + dz * dz

class DspRandom:
    def __init__(self, seed):
        self.seed = seed
        self.seed1 = 0
        self.seed2 = 0
        self.x = 0
        self.y = 0
        self.z = 0
        self.w = 0
        self._state = seed
        self.state = seed

    @property
    def state(self):
        return self._state

    @state.setter
    def state(self, value):
        self._state = value
        if self._state == 0:
            self._state = 2147483647
        self.seed1 = self._state
        self.seed2 = self.seed1
        self.x = self.seed1
        self.y = 842502087
        self.z = self.seed1 * 2
        self.w = self.z + self.y
        if self.y == 0:
            self.y = 1
        if self.z == 0:
            self.z = 1
        if self.w == 0:
            self.w = 1
        self.next_f64()
        self.next_f64()
        self.next_f64()
        self.next_f64()

    def next(self):
        t = self.x ^ (self.x << 11)
        self.x = self.y
        self.y = self.z
        self.z = self.w
        self.w = self.w ^ (self.w >> 19) ^ (t ^ (t >> 8))
        return self.w

    def next_f64(self):
        return (self.next() & 2147483647) / 2147483647.0

    def next_f32(self):
        return (self.next() & 2147483647) / 2147483647.0

    def next_seed(self):
        return self.next()

def generate_temp_poses(seed, target_count, iter_count, min_dist, min_step_len, max_step_len, flatten):
    tmp_poses = []
    actual_iter_count = max(1, min(16, iter_count))
    random_poses(
        tmp_poses,
        seed,
        target_count * actual_iter_count,
        min_dist,
        max_step_len - min_step_len,
        flatten,
    )

    # This is a bit of a weird way to do this, but it's a direct port of the Rust code
    # It removes elements from the end of the list until the list is the desired size
    # and the indexes are multiples of iter_count
    index = len(tmp_poses) - 1
    while index >= 0:
        if index % iter_count != 0:
            tmp_poses.pop(index)
        if len(tmp_poses) <= target_count:
            break
        index -= 1

    return tmp_poses

def random_poses(tmp_poses, seed, max_count, min_dist, step_diff, flatten):
    rand = DspRandom(seed)
    num1 = rand.next_f64()
    tmp_drunk = []
    tmp_poses.append(Vector3.zero())
    num2 = 6
    num3 = 8
    num4 = float(num3 - num2)
    num5 = int(num1 * num4 + float(num2))
    for _ in range(num5):
        for _ in range(256):
            num7 = rand.next_f64() * 2.0 - 1.0
            num8 = (rand.next_f64() * 2.0 - 1.0) * flatten
            num9 = rand.next_f64() * 2.0 - 1.0
            num10 = rand.next_f64()
            d = num7 * num7 + num8 * num8 + num9 * num9
            if 1e-8 <= d <= 1.0:
                num11 = math.sqrt(d)
                num12 = (num10 * step_diff + min_dist) / num11
                pt = Vector3(num7 * num12, num8 * num12, num9 * num12)
                if not check_collision(tmp_poses, pt, min_dist):
                    tmp_drunk.append(pt)
                    tmp_poses.append(pt)
                    if len(tmp_poses) >= max_count:
                        return
                    break
    for _ in range(256):
        for i in range(len(tmp_drunk)):
            if rand.next_f64() <= 0.7:
                for _ in range(256):
                    num15 = rand.next_f64() * 2.0 - 1.0
                    num16 = (rand.next_f64() * 2.0 - 1.0) * flatten
                    num17 = rand.next_f64() * 2.0 - 1.0
                    num18 = rand.next_f64()
                    d = num15 * num15 + num16 * num16 + num17 * num17
                    if 1e-8 <= d <= 1.0:
                        num19 = math.sqrt(d)
                        num20 = (num18 * step_diff + min_dist) / num19
                        new_pt = Vector3(
                            tmp_drunk[i].x + num15 * num20,
                            tmp_drunk[i].y + num16 * num20,
                            tmp_drunk[i].z + num17 * num20,
                        )
                        if not check_collision(tmp_poses, new_pt, min_dist):
                            tmp_drunk[i] = new_pt
                            tmp_poses.append(new_pt)
                            if len(tmp_poses) >= max_count:
                                return
                            break

from enum import Enum

class StarType(Enum):
    MainSeqStar = "MainSeqStar"
    GiantStar = "GiantStar"
    WhiteDwarf = "WhiteDwarf"
    NeutronStar = "NeutronStar"
    BlackHole = "BlackHole"

class SpectrType(Enum):
    M = "M"
    K = "K"
    G = "G"
    F = "F"
    A = "A"
    B = "B"
    O = "O"
    X = "X"

class GameDesc:
    def __init__(self, seed, star_count, resource_multiplier):
        self.seed = seed
        self.star_count = star_count
        self.resource_multiplier = resource_multiplier

class Star:
    def __init__(self, game_desc, index, seed, position, star_type, spectr_type):
        self.game_desc = game_desc
        self.index = index
        self.seed = seed
        self.position = position
        self.star_type = star_type
        self.spectr_type = spectr_type
        self.name = "" # Will be generated later
        self.planets = [] # Will be generated later

class StarWithPlanets:
    def __init__(self, star):
        self.star = star
        self.name = ""
        self.planets = []

    def load_planets(self):
        # TODO: Implement planet generation
        pass

class Galaxy:
    def __init__(self, seed, stars):
        self.seed = seed
        self.stars = stars

def check_collision(tmp_poses, pt, min_dist):
    min_dist_sq = min_dist * min_dist
    for pt1 in tmp_poses:
        if pt1.distance_sq_from(pt) < min_dist_sq:
            return True
    return False

def generate_stars(game_desc):
    galaxy_seed = game_desc.seed
    rand = DspRandom(galaxy_seed)
    tmp_poses = generate_temp_poses(
        rand.next_seed(),
        game_desc.star_count,
        4,
        2.0,
        2.3,
        3.5,
        0.18,
    )
    star_count = len(tmp_poses)

    num1 = rand.next_f32()
    num2 = rand.next_f32()
    num3 = rand.next_f32()
    num4 = rand.next_f32()
    num5 = math.ceil(0.01 * star_count + num1 * 0.3)
    num6 = math.ceil(0.01 * star_count + num2 * 0.3)
    num7 = math.ceil(0.016 * star_count + num3 * 0.4)
    num8 = math.ceil(0.013 * star_count + num4 * 1.3)
    num9 = star_count - num5
    num10 = num9 - num6
    num11 = num10 - num7
    num12 = (num11 - 1) // num8
    num13 = num12 // 2

    stars = []

    for index, position in enumerate(tmp_poses):
        seed = rand.next_seed()
        if index == 0:
            stars.append(StarWithPlanets(Star(
                game_desc,
                0,
                seed,
                Vector3.zero(),
                StarType.MainSeqStar,
                SpectrType.X,
            )))
        else:
            need_spectr = SpectrType.X
            if index == 3:
                need_spectr = SpectrType.M
            elif index == num11 - 1:
                need_spectr = SpectrType.O

            need_type = StarType.MainSeqStar
            if index % num12 == num13:
                need_type = StarType.GiantStar
            elif index >= num9:
                need_type = StarType.BlackHole
            elif index >= num10:
                need_type = StarType.NeutronStar
            elif index >= num11:
                need_type = StarType.WhiteDwarf

            stars.append(StarWithPlanets(Star(
                game_desc,
                index,
                seed,
                position,
                need_type,
                need_spectr,
            )))
    return stars

def create_galaxy(game_desc):
    stars = generate_stars(game_desc)
    names = []

    for sp in stars:
        name = random_name(sp.star.seed, sp.star, names)
        sp.name = name
        names.append(name)
        sp.load_planets()

    return Galaxy(game_desc.seed, stars)

def random_name(seed, star, existing_names):
    # TODO: Implement name generation
    return f"Star {seed}"

import argparse
import json

def to_dict(obj):
    if isinstance(obj, Enum):
        return obj.value
    if isinstance(obj, (str, int, float, bool)):
        return obj
    if isinstance(obj, dict):
        return {k: to_dict(v) for k, v in obj.items() if k not in ['_name_', '__objclass__', '_sort_order_']}
    if isinstance(obj, list):
        return [to_dict(v) for v in obj]
    if hasattr(obj, '__dict__'):
        return to_dict(obj.__dict__)
    return str(obj)

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("--seed", type=int, help="The seed to generate.")
    parser.add_argument("--generate-all", action="store_true", help="Generate all seeds.")
    args = parser.parse_args()

    if args.seed is not None:
        game_desc = GameDesc(args.seed, 64, 1.0)
        galaxy = create_galaxy(game_desc)
        print(json.dumps(to_dict(galaxy)))
    elif args.generate_all:
        with open("galaxy_data.jsonl", "w") as f:
            for seed in range(100_000_000):
                if seed % 10000 == 0:
                    print(f"Generated {seed} seeds...")
                game_desc = GameDesc(seed, 64, 1.0)
                galaxy = create_galaxy(game_desc)
                f.write(json.dumps(to_dict(galaxy)) + "\n")
    else:
        # Test the DspRandom class
        rand = DspRandom(12345)
        for i in range(10):
            print(rand.next_f64())
