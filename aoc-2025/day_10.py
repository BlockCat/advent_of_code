from pulp import * 

def parseLine(input: str):
    split = input.split(" ")
    buttons = [parseContainer(button) for button in split[1:len(split)-1]]
    targets = parseContainer(split[len(split)-1])

    return buttons, targets

    
def parseContainer(input: str) -> list[int]:
    input = input[1:len(input)-1]
    return [int(n) for n in input.split(",")]


filePath = "inputs/day_10.txt"
with open(filePath) as file:
    machines = [parseLine(line.strip()) for line in file]


total = 0
for index, machine in enumerate(machines):
    buttons, targets = machine

    problem = LpProblem(f"machine_{index}", LpMinimize)

    buttonLp = [LpVariable(f"m{index}b{i}", cat=LpInteger) for i,b in enumerate(buttons)]

    for lp in buttonLp:        
        problem += lp >= 0

    problem += lpSum(buttonLp)

    for i, target in enumerate(targets):
        buttonIndeces: list[int] = []
        for j, button in enumerate(buttons):
            if i in button:
                buttonIndeces.append(j)
        buttonLpNeeded = [buttonLp[blp] for blp in buttonIndeces]
        problem += target == lpSum(buttonLpNeeded)

    status = problem.solve()

    presses = [int(value(lp)) for lp in buttonLp]


    total += sum(presses)
    



print("result", total)