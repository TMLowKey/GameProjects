extends Area2D
@export var speed = 400
@export var velocity = Vector2.ONE * speed
var angular_speed = PI
var screen_size



# Called when the node enters the scene tree for the first time.
func _ready():
	set_position(Vector2(300,200))
	#set_rotation_degrees(randf_range(0,360))
	screen_size = get_viewport_rect().size
	print(screen_size)
	print(screen_size.x)
	print(position)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	position += velocity * delta
	
	print(velocity)
	print(position)
	

func bottom_bar_entered(area):
	velocity.y = velocity.y * -1
	print("Bar")

func top_bar_entered(area):
	velocity.y = velocity.y * -1
	print("Bar")
func left_bar_entered(area):
	velocity.x = velocity.x * -1
	print("Bar")
func right_bar_entered(area):
	velocity.x = velocity.x * -1
	print("Bar")
