extends Node2D

const STAGE_START := -200

@onready var stage_borders: TextureRect = $CanvasLayer/Panel/StageBorders
@onready var player_1 = $CanvasLayer/Panel/StageBorders/Player1
@onready var player_2 = $CanvasLayer/Panel/StageBorders/Player2
@onready var p1_round1 = $CanvasLayer/Rounds/P1Round1
@onready var p1_round2 = $CanvasLayer/Rounds/P1Round2
@onready var p1_round3 = $CanvasLayer/Rounds/P1Round3
@onready var p2_round3 = $CanvasLayer/Rounds/P2Round3
@onready var p2_round2 = $CanvasLayer/Rounds/P2Round2
@onready var p2_round1 = $CanvasLayer/Rounds/P2Round1
@onready var p1_guard1 = $CanvasLayer/Guards/P1Guard1
@onready var p1_guard2 = $CanvasLayer/Guards/P1Guard2
@onready var p1_guard3 = $CanvasLayer/Guards/P1Guard3
@onready var p2_guard3 = $CanvasLayer/Guards/P2Guard3
@onready var p2_guard2 = $CanvasLayer/Guards/P2Guard2
@onready var p2_guard1 = $CanvasLayer/Guards/P2Guard1

@onready var ai_controller_p1: FighterAiController = $AIControllerP1
@onready var ai_controller_p2: FighterAiController = $AIControllerP2

@onready var round0 = preload("res://art/round_0.png")
@onready var round1 = preload("res://art/round_1.png")

var cont := true
var simulator: Match

@export var graphics : bool = true
@export var player1_type: AIController2D.ControlModes = AIController2D.ControlModes.INHERIT_FROM_SYNC
@export var player2_type: AIController2D.ControlModes = AIController2D.ControlModes.INHERIT_FROM_SYNC

var p1_input_type: PlayerType
var p2_input_type: PlayerType

# Called when the node enters the scene tree for the first time.
func _ready():
	ai_controller_p1.control_mode = player1_type
	ai_controller_p2.control_mode = player2_type
	
	p1_input_type = PlayerType.Ai1 if player1_type != AIController2D.ControlModes.HUMAN else PlayerType.Player1
	p2_input_type = PlayerType.Ai2 if player2_type != AIController2D.ControlModes.HUMAN else PlayerType.Player2
	
	simulator = Match.gd_new(p1_input_type != PlayerType.Player1, p2_input_type != PlayerType.Player2)

var p1_prev_mov: int = 0
var p2_prev_mov: int = 0
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _physics_process(delta):
	var p1_input := get_inputs(p1_input_type)
	var p2_input := get_inputs(p2_input_type)
	
	if cont:
		var res := simulator.frame_update(p1_input, p2_input)
		cont = res == Result.Continue || res == Result.Pause
		
		if graphics:
			var stage_size = stage_borders.size
			player_1.position.x = ((simulator.p1_pos() + STAGE_START) as float) * (stage_size.x / 1872.0)
			player_2.position.x = ((simulator.p2_pos() + STAGE_START) as float) * (stage_size.x / 1872.0)
			
			player_1.texture = load("res://art/fighter/" + simulator.p1_sprite() + ".png")
			player_2.texture = load("res://art/fighter/" + simulator.p2_sprite() + ".png")
			
			round_ui_update(simulator.p1_wins(), simulator.p2_wins())
			guard_ui_update(simulator.player_guard(true), simulator.player_guard(false))
			
			for audio in simulator.audio():
				play_audio(audio)
		
		update_ai_reward(res, ai_controller_p1)
		update_ai_reward(res, ai_controller_p2)
	else:
		simulator.new_round()
		ai_controller_p1.needs_reset = true
		ai_controller_p2.needs_reset = true
		if simulator.continues():
			cont = true
		else:
			ai_controller_p1.done = true
			ai_controller_p2.done = true
			simulator = Match.gd_new(p1_input_type != PlayerType.Player1, p2_input_type != PlayerType.Player2)
	
	pass

enum Result {
	Continue,
	Pause,
	Player1,
	Player2,
	Draw,
	Timeout,
}

enum PlayerType {
	Player1,
	Player2,
	Ai1,
	Ai2,
}

func get_inputs(type: PlayerType) -> FgInput:
		match type:
			PlayerType.Player1:
				var p1_movement := (Input.is_action_pressed("p1_forward") as int) - (Input.is_action_pressed("p1_backward") as int)
				var p1_movement_pres := p1_movement if p1_prev_mov != p1_movement else 0
				p1_prev_mov = p1_movement
				var p1_attack_press := Input.is_action_just_pressed("p1_attack")
				var p1_attack_hold := Input.is_action_pressed("p1_attack")
				var p1_input := FgInput.gd_new(p1_movement, p1_movement_pres, p1_attack_press, p1_attack_hold)
				return p1_input
			PlayerType.Player2:
				var p2_movement := (Input.is_action_pressed("p2_forward") as int) - (Input.is_action_pressed("p2_backward") as int)
				var p2_movement_pres := p2_movement if p2_prev_mov != p2_movement else 0
				p2_prev_mov = p2_movement
				var p2_attack_press := Input.is_action_just_pressed("p2_attack")
				var p2_attack_hold := Input.is_action_pressed("p2_attack")
				var p2_input := FgInput.gd_new(p2_movement, p2_movement_pres, p2_attack_press, p2_attack_hold)
				return p2_input
			PlayerType.Ai1:
				var ai := ai_controller_p1
				var index := ai.index
				
				if  index >= len(ai.move_actions):
					return  FgInput.gd_new(0, 0, false, false)
					
				var p1_movement :int = ai.move_actions[index]
				var p1_movement_pres :int = p1_movement if p1_prev_mov != p1_movement else 0
				p1_prev_mov = p1_movement
				var p1_attack_press :bool = ai.attack_presses[index]
				var p1_attack_hold :bool = ai.attack_holds[index]
				var p1_input := FgInput.gd_new(p1_movement, p1_movement_pres, p1_attack_press, p1_attack_hold)
				
				ai.index += 1
				
				return p1_input
			PlayerType.Ai2:
				var ai := ai_controller_p2
				var index := ai.index
				
				if  index >= len(ai.move_actions):
					return  FgInput.gd_new(0, 0, false, false)
					
				var p2_movement :int = ai.move_actions[index]
				var p2_movement_pres :int = p2_movement if p2_prev_mov != p2_movement else 0
				p2_prev_mov = p2_movement
				var p2_attack_press :bool = ai.attack_presses[index]
				var p2_attack_hold :bool = ai.attack_holds[index]
				var p2_input := FgInput.gd_new(p2_movement, p2_movement_pres, p2_attack_press, p2_attack_hold)
				
				ai.index += 1
				
				return p2_input
			_:
				assert(false, "wait what???")
				return null

func play_audio(audio_path: String):
	var audio_stream = load("res://audio/" + audio_path + ".wav") as AudioStream
	if audio_stream:
		var audio_player = AudioStreamPlayer.new()
		audio_player.stream = audio_stream
		add_child(audio_player)
		audio_player.play()
		audio_player.finished.connect(func() -> void:
			audio_player.queue_free()
		)

func round_ui_update(p1: int, p2: int):
	p1_round3.texture = round1 if p1 >= 3 else round0
	p1_round2.texture = round1 if p1 >= 2 else round0
	p1_round1.texture = round1 if p1 >= 1 else round0
	
	p2_round3.texture = round1 if p2 >= 3 else round0
	p2_round2.texture = round1 if p2 >= 2 else round0
	p2_round1.texture = round1 if p2 >= 1 else round0
	pass

func guard_ui_update(p1: int, p2: int):
	p1_guard3.visible = p1 >= 3
	p1_guard2.visible = p1 >= 2
	p1_guard1.visible = p1 >= 1
	
	p2_guard3.visible = p2 >= 3
	p2_guard2.visible = p2 >= 2
	p2_guard1.visible = p2 >= 1
	pass

func get_player_obs(p1: bool) -> Array:
	return simulator.player_obs(p1)

func get_punish_obs(p1: bool) -> Array:
	return simulator.punish_obs(p1)

func game_over():
	simulator = Match.gd_new(p1_input_type != PlayerType.Player1, p2_input_type != PlayerType.Player2)
	ai_controller_p1.reset()
	ai_controller_p2.reset()

func update_ai_reward(res: Result, ai: FighterAiController):
	# step penalty
	if res == Result.Continue:
		ai.reward -= 0.15
		# corner
		ai.reward -= 0.50 if simulator.player_relative_pos(ai.player1) < 200 else 0
		# distance penalty
		ai.reward -= max(simulator.player_distance(), 710) as float ** 0.4 - 13.82
	
	ai.reward += 0.30 if simulator.player_hold(ai.player1) >= 30 else 0
	# ai.reward += 20 if simulator.player_counter(!ai.player1) else 0
	ai.reward += 20 if simulator.player_hit(!ai.player1) else 0
	ai.reward += 10 if simulator.player_block(!ai.player1) else 0
	ai.reward += 10 if simulator.player_block(ai.player1) else 0
	ai.reward -= 30 if simulator.player_block_ender(!ai.player1) else 0
	ai.reward += 30 if simulator.player_guard_break(!ai.player1) else 0
	ai.reward += 100 if simulator.player_dead(!ai.player1) else 0
	
	# ai.reward -= 10 if simulator.player_counter(ai.player1) else 0
	ai.reward -= 20 if simulator.player_hit(ai.player1) else 0
	ai.reward -= 25 if simulator.player_dead(ai.player1) else 0
	
