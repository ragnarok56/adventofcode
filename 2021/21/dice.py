with open('in') as f:
    players = [int(l.strip()[-1]) for l in f.readlines()]

def part1():
    class DeterministicDie:
        cur = 0
        rolls = 0

        def roll(self):
            self.cur += 1
            self.rolls += 1
            return self.cur

    scores = [0] * len(players)
    board_size = 10
    dice = DeterministicDie()

    winner = None

    while winner is None:
        for i, p in enumerate(players):
            moves = dice.roll() + dice.roll() + dice.roll()

            new_pos = players[i] + moves

            players[i] = board_size if new_pos % board_size == 0 else new_pos % board_size

            scores[i] += players[i]

            if scores[i] >= 1000:
                winner = i
                break

    print(f'dice rolls: {dice.rolls}')
    loser = [i for i,_ in enumerate(players) if i != winner][0]
    print(f'loser score: {scores[loser]}')

    print(f'rolls * loser score: {dice.rolls * scores[loser]}')
    
# part1()

def part2():
    board_size = 10

    roll_outcomes = {
        3: 1,
        4: 3,
        5: 6,
        6: 7,
        7: 6,
        8: 3,
        9: 1
    }

    cache = {}          
    
    def key(p1, p2, active):
        return (p1[0],p1[1],p2[0],p2[1], active)

    def calc_pos(pm):
        p = pm[0]
        m = pm[1][0]
    
        p_pos = p[0]
        score = p[1]
        new_pos = board_size if (p_pos + m) % board_size == 0 else (p_pos + m) % board_size
        new_score = score + new_pos

        return (new_pos, new_score)

    target_score = 21
    def delve(p1, p2, active):
        if key(p1, p2, active) in cache:
            return cache[key(p1, p2, active)]
        # print(f'{p1} vs {p2}')
        p1_wins = 0
        p2_wins = 0
        for rolls in roll_outcomes.items():
            if active == 1:
                new_p1 = calc_pos((p1, rolls))
                if new_p1[1] >= target_score:
                    p1_wins += rolls[1]
                else:
                    # print(f'delve - p1e: {new_p1[1]}')
                    res = delve(new_p1, p2, 2)
                    p1_wins += res[0] * rolls[1]
                    p2_wins += res[1] * rolls[1]
                    # print(f'delve - p1x: {p1_wins}, {p2_wins}')
            else:
                new_p2 = calc_pos((p2, rolls))
                if new_p2[1] >= target_score:
                    p2_wins += rolls[1]
                else:
                    # print(f'delve - p2e: {new_p2[1]}')
                    res = delve(p1, new_p2, 1)
                    p1_wins += res[0] * rolls[1]
                    p2_wins += res[1] * rolls[1]
                    # print(f'delve - p2x: {p1_wins}, {p2_wins}')
            # print(f'wins: {p1_wins}, {p2_wins}')

        # print(f'setting cache for {key(p1, p2, active)} = {(p1_wins, p2_wins)}')
        cache[key(p1, p2, active)] = (p1_wins, p2_wins)
        return (p1_wins, p2_wins)
            
    
    wins = delve((players[0], 0, 0), (players[1], 0, 0), 1)
    
    print(f'p1: {wins[0]}, p2: {wins[1]}, win %: {wins[0] / (wins[0] + wins[1]) * 100}')


part2()