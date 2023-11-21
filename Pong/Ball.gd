extends Area2D
@export var speed = 400
@export var velocity = Vector2.ZERO
var angular_speed = PI
var screen_size



# Called when the node enters the scene tree for the first time.
func _ready():
	#set_rotation_degrees(randf_range(0,360))
	set_rotation_degrees(-10)
	screen_size = get_viewport_rect().size
	print(screen_size)
	print(screen_size.x)
	print(position)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	velocity = Vector2.UP.rotated(rotation) * speed
	position += velocity * delta
	
	#if not((position.x >= screen_size.x) or (position.y >= screen_size.y)):
	#	var velocity = Vector2.LEFT * speed
	#	position += velocity * delta
	#else:
	#	print("Touch border")
	
		# Detekce nárazu na stěny a změna rotace a chování míčku
	if (position.y <= 0) or (position.y >= screen_size.y):
		if rotation_degrees > 0:
			rotation_degrees = (45 + 90) % 360
		else:
			rotation_degrees = (90 + 90) % 360
			
	if (position.x <= 0):
		speed = 0
		position.x = 0
		
	if (position.x >= screen_size.x):
		speed = 0
		position.x = screen_size.x


func bottom_bar_entered(area):
	if rotation_degrees > 0:
		rotation_degrees = (45 + 90) % 360
	else:
		rotation_degrees = (90 + 90) % 360


func top_bar_entered(area):
	print(rotation_degrees)
	if rotation_degrees > 0:
		rotation_degrees = (45 + 90) % 360
	else:
		rotation_degrees = (90 + 90) % 360
		
func left_bar_entered(area):
	print(rotation_degrees)
	speed = 0

func right_bar_entered(area):
	print(rotation_degrees)
	speed = 0
