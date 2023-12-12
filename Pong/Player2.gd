extends Area2D
@export var speed = 400
var screen_size

# Called when the node enters the scene tree for the first time.
func _ready():
	screen_size = get_viewport_rect().size


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	var velocity = Vector2.ZERO
	if Input.is_action_pressed("p2_move_up"):
		velocity.y -= 1
		#print("Up")
	if Input.is_action_pressed("p2_move_down"):
		velocity.y += 1
		#print("Down")
	
	if velocity.length() > 0:
	# Vector normalization so player is not moving faster while traveling diagonaly
		velocity = velocity.normalized() * speed
		
	position += velocity * delta
	# Limit user movement within screensize + size paddle, so it stop infront of border 
	position = position.clamp(Vector2.ZERO + Vector2(0,65), screen_size - Vector2(0,65))
	
	#0
	#if (position.y == 228 or position.y == 0):
	#	velocity = 0
	#	print("border")
