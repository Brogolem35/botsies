extends AIController2D
class_name FighterAiController

@onready var battle_scene = $".."

@export var player1: bool = true

var prev: Array[Array] = []
const prev_limit = 12
var punish_prev: Array[Array] = []
var memory: Array[Array] = []
const memory_limit = 5

const action_size := 5
var move_actions : Array = [0, 0, 0, 0, 0]
var attack_presses: Array = [false, false, false, false, false]
var attack_holds: Array = [false, false, false, false, false]
var index: int = 0
var holds: bool = false
var last_action: Dictionary = {
		"fwalk": 0,
		"bwalk": 0,
		"fdash": 0,
		"bdash": 0,
		"nattack": 0,
		"mattack": 0,
		"nhold": 0,
		"mhold": 0,
		"nrelease": 0,
		"mrelease": 0,
		"none": 0,
		}

func _physics_process(_delta):
	n_steps += 1

	var obs = battle_scene.get_player_obs(player1)
	
	while(len(memory) < memory_limit):
		memory.append(obs)
	
	prev.append(obs)
	punish_prev.append(battle_scene.get_punish_obs(player1))
	if len(prev) > prev_limit:
		memory.append(prev.pop_front())
		punish_prev.pop_front()
	if len(memory) > memory_limit:
		memory.pop_front()

func get_obs() -> Dictionary:
	
	while(len(memory) < memory_limit):
		var obs = battle_scene.get_player_obs(player1)
		memory.append(obs)
	
	var res = []
	for m in memory:
		res.append_array(m)
	
	res.append(holds)
	res.append_array(last_action.values())
	
	var punish = [0, 0, 0, 0] if len(punish_prev) < memory_limit else punish_prev[memory_limit - 1]
	res.append_array(punish)
	
	return {"obs":res}

func get_reward():
	var current_reward = reward
	reward = 0  # reset the reward to zero checked every decision step
	return current_reward
	
func get_action_space() -> Dictionary:
	return {
		"fwalk": {
			"size": 2,
			"action_type": "discrete"
		},
		"bwalk": {
			"size": 2,
			"action_type": "discrete"
		},
		"fdash": {
			"size": 2,
			"action_type": "discrete"
		},
		"bdash": {
			"size": 2,
			"action_type": "discrete"
		},
		"nattack": {
			"size": 2,
			"action_type": "discrete"
		},
		"mattack": {
			"size": 2,
			"action_type": "discrete"
		},
		"nhold": {
			"size": 2,
			"action_type": "discrete"
		},
		"mhold": {
			"size": 2,
			"action_type": "discrete"
		},
		"nrelease": {
			"size": 2,
			"action_type": "discrete"
		},
		"mrelease": {
			"size": 2,
			"action_type": "discrete"
		},
		"none": {
			"size": 2,
			"action_type": "discrete"
		},
		}
	
func set_action(action) -> void:
	const FORWARD = 1;
	const BACKWARD = -1;
	
	index = 0
	
	for key in last_action.keys():
		last_action[key] = 0
	for key in action.keys():
		if action[key] != 0:
			last_action[key] = action[key]
			break
	
	if action["fwalk"] != 0:
		move_actions = range(0, action_size).map(func(x): return FORWARD)
		attack_presses = range(0, action_size).map(func(x): return false)
		attack_holds = range(0, action_size).map(func(x): return holds)
	elif action["bwalk"] != 0:
		move_actions = range(0, action_size).map(func(x): return BACKWARD)
		attack_presses = range(0, action_size).map(func(x): return false)
		attack_holds = range(0, action_size).map(func(x): return holds)
	elif action["fdash"] != 0:
		move_actions = range(0, action_size).map(func(x): return FORWARD if x == 0 || x == 2 else 0)
		attack_presses = range(0, action_size).map(func(x): return false)
		attack_holds = range(0, action_size).map(func(x): return holds)
	elif action["bdash"] != 0:
		move_actions = range(0, action_size).map(func(x): return BACKWARD if x == 0 || x == 2 else 0)
		attack_presses = range(0, action_size).map(func(x): return false)
		attack_holds = range(0, action_size).map(func(x): return holds)
	elif action["nattack"] != 0:
		if holds:
			no_action()
		
		move_actions = range(0, action_size).map(func(x): return 0)
		attack_presses = range(0, action_size).map(func(x): return x == 0)
		attack_holds = range(0, action_size).map(func(x): return holds)
	elif action["mattack"] != 0:
		if holds:
			no_action()
		
		move_actions = range(0, action_size).map(func(x): return BACKWARD)
		attack_presses = range(0, action_size).map(func(x): return x == 0)
		attack_holds = range(0, action_size).map(func(x): return holds)
	elif action["nhold"] != 0:
		if holds:
			no_action()
		
		holds = true
		
		move_actions = range(0, action_size).map(func(x): return 0)
		attack_presses = range(0, action_size).map(func(x): return x == 0)
		attack_holds = range(0, action_size).map(func(x): return true)
	elif action["mhold"] != 0:
		if holds:
			no_action()
		
		holds = true
		
		move_actions = range(0, action_size).map(func(x): return BACKWARD)
		attack_presses = range(0, action_size).map(func(x): return x == 0)
		attack_holds = range(0, action_size).map(func(x): return true)
	elif action["nrelease"] != 0:
		if !holds:
			no_action()
		
		holds = false
		
		move_actions = range(0, action_size).map(func(x): return 0)
		attack_presses = range(0, action_size).map(func(x): return false)
		attack_holds = range(0, action_size).map(func(x): return false)
	elif action["mrelease"] != 0:
		if !holds:
			no_action()
		
		holds = false
		
		move_actions = range(0, action_size).map(func(x): return BACKWARD)
		attack_presses = range(0, action_size).map(func(x): return false)
		attack_holds = range(0, action_size).map(func(x): return false)
	elif action["none"] != 0:
		no_action()

func no_action():
	move_actions = range(0, action_size).map(func(x): return 0)
	attack_presses = range(0, action_size).map(func(x): return false)
	attack_holds = range(0, action_size).map(func(x): return holds)

func reset():
	n_steps = 0
	needs_reset = false
	prev.clear()
	punish_prev.clear()
	memory.clear()
	
	index = 0
	holds = false
	move_actions.clear()
	attack_presses.clear()
	attack_holds.clear()
