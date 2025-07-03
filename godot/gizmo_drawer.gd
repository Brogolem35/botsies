extends Node2D

@onready var battle_scene = $"../.."

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	queue_redraw()

var collision_p1 = Vector2.DOWN
var collision_p2 = Vector2.DOWN
var collision_size = Vector2(125, -153)
var collision_color = Color(0.3, 0.3, 0.3, 0.7)

var hurtbox_p1 = []
var hurtbox_p2 = []
var hurtbox_color = Color(0.3, 0.3, 0.3, 0.7)

# Called every frame to draw
func _draw():
	draw_rect(Rect2(collision_p1 + Vector2(400, 500), collision_size * Vector2(1, 1)), collision_color)
	draw_rect(Rect2(collision_p2 + Vector2(400, 500), collision_size * Vector2(-1, 1)), collision_color)
	
