from OpenGL.GL import *
from OpenGL.GLU import *
from OpenGL.GLUT import *
import pygame
from pygame.locals import *
from gameobjects.matrix44 import *
import os, sys, random, math
from texutil import *

if sys.platform == ('win32' or 'win64'):
    os.environ['SDL_VIDEO_CENTERED'] = '1'

from cube_object import *
import GL

pygame.init()
screen = [800,600]
GL.gl_init(*screen)
pygame.display.set_caption("Rubik's Cube - Press 'i' for instructions")
pygame.display.set_icon(pygame.image.load("Images/RubiksCubeIcon.png").convert_alpha())

def vec3(vec2,axis):
    vec3=[0,0,0]
    if axis[0]!=0:
        vec3[1],vec3[2]=vec2
        vec3[0]=axis[0]+1
    elif axis[1]!=0:
        vec3[0],vec3[2]=vec2
        vec3[1]=axis[1]+1
    elif axis[2]!=0:
        vec3[0],vec3[1]=vec2
        vec3[2]=axis[2]+1
    else: print "ERROR"
    return vec3

def rotate_blocks(direction1,direction2,axis):
    global cube
    block_sequence=((0,0),(1,0),(2,0),(2,1),(2,2),(1,2),(0,2),(0,1))
    block_sequence=block_sequence[::direction1]
    t1=cube.sides[vec3(block_sequence[0],axis)]
    t2=cube.sides[vec3(block_sequence[1],axis)]
    x1=cube.rotations[vec3(block_sequence[0],axis)]
    x2=cube.rotations[vec3(block_sequence[1],axis)]
    for i in xrange(6):
        cube.sides[vec3(block_sequence[i],axis)]= \
            cube.sides[vec3(block_sequence[i+2],axis)]
        cube.rotations[vec3(block_sequence[i],axis)] = \
            cube.rotations[vec3(block_sequence[i+2],axis)]
    cube.sides[vec3(block_sequence[6],axis)]=t1
    cube.sides[vec3(block_sequence[7],axis)]=t2
    cube.rotations[vec3(block_sequence[6],axis)]=x1
    cube.rotations[vec3(block_sequence[7],axis)]=x2
    if axis[0]!=0:
        rot=Matrix44.x_rotation(radians(90.*direction2))
    elif axis[1]!=0:
        rot=Matrix44.y_rotation(radians(90.*direction2))
    elif axis[2]!=0:
        rot=Matrix44.z_rotation(radians(90.*direction2))
    for i in xrange(8):
        cube.rotations[vec3(block_sequence[i],axis)]=\
            rot*cube.rotations[vec3(block_sequence[i],axis)]
    cube.rotations[vec3((1,1),axis)]=\
        rot*cube.rotations[vec3((1,1),axis)]

def rotate_face(direction1,direction2,axis):
    global cube, rotate, view_distance,viewmode,animations
    global instructions, display_instructions
    if animations:
        for a in xrange(0,90*direction2,direction2):
            cube.draw(rotate[0],rotate[1],view_distance,viewmode,
                      a,axis[0],axis[1],axis[2])
            if display_instructions:
                instructions.draw(rotate[0],rotate[1],view_distance)
            pygame.display.flip()
    rotate_blocks(direction1,direction2,axis)
    

def shuffle():
    global move_list
    move_list=[]
    possible_moves="lLrRtTfFbBuU"
    last_s=""
    for _ in xrange(random.randint(20,40)):
        s=possible_moves[random.randint(0,len(possible_moves)-1)]
        if not (s.capitalize()==last_s.capitalize() and s!=last_s):
            move_list+=s
            last_s=s
        
        
def get_input():
    global view_distance, rotate, move_list, solving, cube, animations
    global display_instructions, game_running
    for event in pygame.event.get():
        if event.type == QUIT: game_running=False
        if event.type == KEYDOWN:
            if event.key==27: game_running=False
            elif event.key==284: # F3 key
                global viewmode
                if viewmode == 2.0:
                    while True:
                        viewmode -= .03
                        cube.draw(rotate[0],rotate[1],view_distance,viewmode)
                        if display_instructions:
                            instructions.draw(rotate[0],rotate[1],view_distance)
                        pygame.display.flip()
                        if viewmode < 1.0:
                            break
                elif viewmode == 1.0:
                    while True:
                        viewmode += .03
                        cube.draw(rotate[0],rotate[1],view_distance,viewmode)
                        if display_instructions:
                            instructions.draw(rotate[0],rotate[1],view_distance)
                        pygame.display.flip()
                        if viewmode > 2.0:
                            break
                if viewmode < 1.0: viewmode = 1.0
                elif viewmode > 2.0: viewmode = 2.0
            elif event.key==K_F4: shuffle()
            elif event.key==K_i: display_instructions= not display_instructions
            elif event.key==K_a: animations= not animations
            elif event.key==K_F5: solving= not solving
            elif event.key==K_F6:
                rot=Matrix44.y_rotation(radians(90.))
                cube.rotations[1,0,0]=rot*cube.rotations[1,0,0]
                for i in xrange(3):
                    for j in xrange(3):
                        for k in xrange(3):
                            print i,j,k,cube.sides[i,j,k]
                            print cube.rotations[i,j,k]
            else:
                move_list+=event.unicode

    keystate = pygame.key.get_pressed()
    if keystate[K_PAGEUP]: view_distance += 0.1
    if keystate[K_PAGEDOWN]: view_distance -= 0.1
    if keystate[K_END]: view_distance = 9.0;rotate[0] = 0.0;rotate[1] = 0.0
    if keystate[K_KP4]: rotate[1] += 1.0
    if keystate[K_KP6]: rotate[1] -= 1.0
    if keystate[K_KP8]: rotate[0] += 1.0
    if keystate[K_KP2]: rotate[0] -= 1.0

    pressed_mouse=pygame.mouse.get_pressed()
    mickeys=pygame.mouse.get_rel()
    if pressed_mouse[0]:
        rotate[0] += mickeys[1]/3.0
        rotate[1] += mickeys[0]/3.0
    rotate[0]=min(rotate[0],45.)
    rotate[0]=max(rotate[0],-45)
    if rotate[1] >= 360: rotate[1] = 0
    elif rotate[1] <= -360: rotate[1] = 0
        
    pygame.event.clear()

def process_next_move():
    global move_list
    if len(move_list)>0:
        s=move_list[0]
        if s=="l":   rotate_face(-1,1,(-1,0,0))
        elif s=="L": rotate_face(1,-1,(-1,0,0))
        elif s=="t": rotate_face(-1,-1,(0,1,0))
        elif s=="T": rotate_face(1,1,(0,1,0))
        elif s=="r": rotate_face(1,-1,(1,0,0))
        elif s=="R": rotate_face(-1,1,(1,0,0))
        elif s=="F": rotate_face(-1,1,(0,0,1))
        elif s=="f": rotate_face(1,-1,(0,0,1))
        elif s=="b": rotate_face(-1,1,(0,0,-1))
        elif s=="B": rotate_face(1,-1,(0,0,-1))
        elif s=="u": rotate_face(1,1,(0,-1,0))
        elif s=="U": rotate_face(-1,-1,(0,-1,0))

        move_list=move_list[1:]

from solver import *        
        
def main():
    global cube, solving, animations, display_instructions, game_running
    global rotate,view_distance, viewmode, move_list, instructions
    
    instructions=Text_Image("Instructions.png")
    rotate = [20.0,20.0,0.0]
    view_distance = 9.0
    viewmode = 1.0
    cube=RubiksCube()
    move_list=""
    solving=False
    animations=True
    display_instructions=False
    game_running=True

    while game_running:
        get_input()
        process_next_move()
        if solving and len(move_list)==0:
            move_list += solver(cube)
            if len(move_list)==0: solving=False
            
        cube.draw(rotate[0],rotate[1],view_distance,viewmode)
        if display_instructions:
            instructions.draw(rotate[0],rotate[1],view_distance)
        pygame.display.flip()
                
    pygame.quit()
        
if __name__ == '__main__': main()

