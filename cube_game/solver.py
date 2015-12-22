
def solver(cube):
    s=""
    location = cube.find_cubie(10)
    direction=cube.rotations[location]
    dir=direction.get_column(1)
    if not (location==(0,2,2) and int(dir[1])==1):
        if location[1]==2:
            if location==(0,2,0):   s="Lul" #s="L"
            elif location==(2,2,0): s="ruR" #s="b"
            elif location==(2,2,2): s="Rur" #s="f"
            elif location==(0,2,2): s="RuL" #s="F"
        else:
            if location==(0,0,0): s="u"
            if location==(2,0,0): s="uu"
            if location==(2,0,2): s="U"
            if location==(0,0,2):
                print "1. Placing (0,2,2) cubie"
                if int(direction.get_column(0)[1])==-1: s="UFuf"
                elif int(direction.get_column(1)[2])==-1: s="ulUL"
                else: s="FufuuFUf"
        return s
    location = cube.find_cubie(28)
    direction=cube.rotations[location]
    dir=direction.get_column(1)
    if not (location==(2,2,2) and int(dir[1])==1):
        if location[1]==2:
            if location==(0,2,0): s="Lul"
            elif location==(2,2,0): s="ruR"
            elif location==(2,2,2): s="Rur"
        else:
            if location==(0,0,0): s="uu"
            if location==(2,0,0): s="U"
            if location==(0,0,2): s="u"
            if location==(2,0,2):
                print "2. Placing (2,2,2) cubie"
                if int(direction.get_column(1)[2])==-1: s="URur"
                elif int(direction.get_column(1)[0])==-1: s="ufUF"
                else: s="RuruuRUr"
        return s
    location = cube.find_cubie(26)
    direction=cube.rotations[location]
    dir=direction.get_column(1)
    if not (location==(2,2,0) and int(dir[1])==1):
        if location[1]==2:
            if location==(0,2,0): s="Lul"
            elif location==(2,2,0): s="ruR"
        else:
            if location==(0,0,0): s="U"
            if location==(2,0,2): s="u"
            if location==(0,0,2): s="uu"
            if location==(2,0,0):
                print "3. Placing (2,2,0) cubie"
                if int(direction.get_column(1)[0])==-1: s="UBub"
                elif int(direction.get_column(1)[2])==-1: s="urUR"
                else: s="BubuuBUb"
        return s
    location = cube.find_cubie(8)
    direction=cube.rotations[location]
    dir=direction.get_column(1)
    if not (location==(0,2,0) and int(dir[1])==1):
        if location[1]==2:
            if location==(0,2,0): s="Lul"
        else:
            if location==(2,0,0): s="u"
            if location==(2,0,2): s="uu"
            if location==(0,0,2): s="U"
            if location==(0,0,0):
                print "4. Placing (0,2,0) cubie"
                if int(direction.get_column(2)[1])==-1: s="ULul"
                elif int(direction.get_column(0)[1])==-1: s="ubUB"
                else: s="LuluuLUl"
        return s
    location = cube.find_cubie(9)
    direction=cube.rotations[location]
    dir=direction.get_column(1)
    if not (location==(0,2,1) and int(dir[1])==1):
        if location[1]==2:
            if location==(0,2,1): s="BfllbF"
            if location==(1,2,2): s="rLffRl"
            if location==(2,2,1): s="FbrrBf"
            if location==(1,2,0): s="RlbbrL"
        elif location[1]==1:
            if location==(0,1,2):
                if int(direction.get_column(1)[2])==-1: s="tbTuL"
                else: s="TTRtUft"
            if location==(2,1,2):
                if int(direction.get_column(1)[2])==-1: s="lTuFt"
                else: s="tBtUrTT"
            if location==(2,1,0):
                if int(direction.get_column(1)[2])==1: s="LtUbT"
                else: s="TfTuRtt"
            if location==(0,1,0):
                if int(direction.get_column(2)[1])==-1: s="TFtul"
                else: s="TTrTuBT"
        else:
            if location==(1,0,0): s="u"
            if location==(2,0,1): s="uu"
            if location==(1,0,2): s="U"
            if location==(0,0,1):
                print "5. Placing (0,2,1) cubie"
                if int(direction.get_column(0)[1])==-1: s="uBfLbF"
                else: s="BfllbF"
        return s
    location = cube.find_cubie(19)
    direction=cube.rotations[location]
    dir=direction.get_column(1)
    if not (location==(1,2,2) and int(dir[1])==1):
        if location[1]==2:
            if location==(0,2,1): s="BfllbF"
            if location==(1,2,2): s="rLffRl"
            if location==(2,2,1): s="FbrrBf"
            if location==(1,2,0): s="RlbbrL"
        elif location[1]==1:
            if location==(2,1,2):
                if int(direction.get_column(1)[2])==-1: s="tlTuF"
                else: s="TTBtUrt"
            if location==(2,1,0):
                if int(direction.get_column(1)[2])==-1: s="fTuRt"
                else: s="tLtUbTT"
            if location==(0,1,0):
                if int(direction.get_column(1)[2])==1: s="FtUlT"
                else: s="TrTubtt"
            if location==(0,1,2):
                if int(direction.get_column(2)[1])==-1: s="TRtuf"
                else: s="TTbTuLT"
        else:
            if location==(0,0,1): s="u"
            if location==(1,0,0): s="uu"
            if location==(2,0,1): s="U"
            if location==(1,0,2):
                print "5. Placing (1,2,2) cubie"
                if int(direction.get_column(1)[2])==-1: s="uLrFlR"
                else: s="LrfflR"
        return s
    location = cube.find_cubie(27)
    direction=cube.rotations[location]
    dir=direction.get_column(1)
    if not (location==(2,2,1) and int(dir[1])==1):
        if location[1]==2:
            if location==(0,2,1): s="BfllbF"
            if location==(1,2,2): s="rLffRl"
            if location==(2,2,1): s="FbrrBf"
            if location==(1,2,0): s="RlbbrL"
        elif location[1]==1:
            if location==(2,1,0):
                if int(direction.get_column(1)[2])==-1: s="tfTuR"
                else: s="TTLtUbt"
            if location==(0,1,0):
                if int(direction.get_column(1)[2])==-1: s="rTuBt"
                else: s="tFtUlTT"
            if location==(0,1,2):
                if int(direction.get_column(1)[2])==1: s="RtUfT"
                else: s="TbTultt"
            if location==(2,1,2):
                if int(direction.get_column(2)[1])==-1: s="TBtur"
                else: s="TTlTuFT"
        else:
            if location==(1,0,2): s="u"
            if location==(0,0,1): s="uu"
            if location==(1,0,0): s="U"
            if location==(2,0,1):
                print "7. Placing (2,2,1) cubie"
                if int(direction.get_column(1)[0])==-1: s="uFbRfB"
                else: s="FbrrfB"
        return s
    location = cube.find_cubie(17)
    direction=cube.rotations[location]
    dir=direction.get_column(1)
    if not (location==(1,2,0) and int(dir[1])==1):
        if location[1]==2:
            if location==(0,2,1): s="BfllbF"
            if location==(1,2,2): s="rLffRl"
            if location==(2,2,1): s="FbrrBf"
            if location==(1,2,0): s="RlbbrL"
        elif location[1]==1:
            if location==(0,1,0):
                if int(direction.get_column(1)[2])==-1: s="trTuB"
                else: s="TTFtUlt"
            if location==(0,1,2):
                if int(direction.get_column(1)[2])==-1: s="bTuLt"
                else: s="tRtUfTT"
            if location==(2,1,2):
                if int(direction.get_column(1)[2])==1: s="BtUrT"
                else: s="TlTuftt"
            if location==(2,1,0):
                if int(direction.get_column(2)[1])==-1: s="TLtub"
                else: s="TTfTuRT"
        else:
            if location==(2,0,1): s="u"
            if location==(1,0,2): s="uu"
            if location==(0,0,1): s="U"
            if location==(1,0,0):
                print "8. Placing (1,2,0) cubie"
                if int(direction.get_column(2)[1])==-1: s="uRlBrL"
                else: s="RlbbrL"
        return s
    location = cube.find_cubie(5)
    direction=cube.rotations[location]
    dir=direction.get_column(1)
    if not (location==(0,1,0) and int(dir[1])==1):
        if location[1]==1:
            if location==(0,1,0): s="bUBULul"
            if location==(0,1,2): s="lULUFuf"
            if location==(2,1,2): s="fUFURur"
            if location==(2,1,0): s="rURUBub"
        else:
            if location==(1,0,0): s="uu"
            if location==(0,0,1): s="u"
            if location==(2,0,1): s="U"
            if location==(1,0,2):
                print "9. Placing (0,1,0) cubie"
                if int(direction.get_column(0)[1])==-1: s="bUBULul"
                else: s="uLulubUB"
        return s
    location = cube.find_cubie(7)
    direction=cube.rotations[location]
    dir=direction.get_column(1)
    if not (location==(0,1,2) and int(dir[1])==1):
        if location[1]==1:
            if location==(0,1,0): s="bUBULul"
            if location==(0,1,2): s="lULUFuf"
            if location==(2,1,2): s="fUFURur"
            if location==(2,1,0): s="rURUBub"
        else:
            if location==(0,0,1): s="uu"
            if location==(1,0,2): s="u"
            if location==(1,0,0): s="U"
            if location==(2,0,1):
                print "10. Placing (2,0,1) cubie"
                if int(direction.get_column(0)[2])==1: s="lULUFuf"
                else: s="uFufulUL"
        return s
    location = cube.find_cubie(25)
    direction=cube.rotations[location]
    dir=direction.get_column(1)
    if not (location==(2,1,2) and int(dir[1])==1):
        if location[1]==1:
            if location==(0,1,0): s="bUBULul"
            if location==(0,1,2): s="lULUFuf"
            if location==(2,1,2): s="fUFURur"
            if location==(2,1,0): s="rURUBub"
        else:
            if location==(1,0,2): s="uu"
            if location==(2,0,1): s="u"
            if location==(0,0,1): s="U"
            if location==(1,0,0):
                print "11. Placing (1,0,0) cubie"
                if int(direction.get_column(1)[2])==-1: s="fUFURur"
                else: s="uRurufUF"
        return s
    location = cube.find_cubie(23)
    direction=cube.rotations[location]
    dir=direction.get_column(1)
    if not (location==(2,1,0) and int(dir[1])==1):
        if location[1]==1:
            if location==(0,1,0): s="bUBULul"
            if location==(0,1,2): s="lULUFuf"
            if location==(2,1,2): s="fUFURur"
            if location==(2,1,0): s="rURUBub"
        else:
            if location==(2,0,1): s="uu"
            if location==(1,0,0): s="u"
            if location==(1,0,2): s="U"
            if location==(0,0,1):
                print "12. Placing (0,0,1) cubie"
                if int(direction.get_column(1)[0])==-1: s="rURUBub"
                else: s="uBuburUR"
        return s
    location = cube.find_cubie(2)
    if not (location==(0,0,0)):
        print "13. Placing (0,0,0) cubie"
        if location==(0,0,2): s="U"
        if location==(2,0,2): s="uu"
        if location==(2,0,0): s="u"
        return s
    location = cube.find_cubie(4)
    if not (location==(0,0,2)):
        print "14. Placing (0,0,2) cubie"
        if location==(2,0,2): s="uBUbruRBubu" # switch 1 3
        if location==(2,0,0): s="BUbruRBubUBUbruRBubu" # switch 1 2 and 1 3
        return s
    location = cube.find_cubie(22)
    if not (location==(2,0,2)):
        print "15. Placing (2,0,2) cubie"
        if location==(2,0,0): s="BUbruRBubuu" # switch 1 2
        return s
    dir1=cube.rotations[0,0,0]
    dir2=cube.rotations[0,0,2]
    dir3=cube.rotations[2,0,2]
    dir4=cube.rotations[2,0,0]
    d1="0"
    if int(dir1.get_column(1)[2])==1: d1="1"
    if int(dir1.get_column(1)[0])==1: d1="2"
    d2="0"
    if int(dir2.get_column(1)[0])==1: d2="1"
    if int(dir2.get_column(1)[2])==-1: d2="2"
    d3="0"
    if int(dir3.get_column(1)[2])==-1: d3="1"
    if int(dir3.get_column(1)[0])==-1: d3="2"
    d4="0"
    if int(dir4.get_column(1)[0])==-1: d4="1"
    if int(dir4.get_column(1)[2])==1: d4="2"
    os=d1+d2+d3+d4
    os+=os
    if (os[0:4]=="0222" or os[0:2]=="21" or os[0:2]=="01"): s="FUfUFUUfUU"
    if (os[1:5]=="0222" or os[1:3]=="21" or os[1:3]=="01"): s="RUrURUUrUU"
    if (os[2:6]=="0222" or os[2:4]=="21" or os[2:4]=="01"): s="BUbUBUUbUU"
    if (os[3:7]=="0222" or os[3:5]=="21" or os[3:5]=="01"): s="LUlULUUlUU"
    if s != "": return s
    loc1=cube.find_cubie(3)
    loc2=cube.find_cubie(13)
    loc3=cube.find_cubie(21)
    loc4=cube.find_cubie(11)
    if not (loc1==(0,0,1) and loc2==(1,0,2) and loc3==(2,0,1) and loc4==(1,0,0)):
        if loc1==(0,0,1): s="fBLFbuufBLFb"
        elif loc2==(1,0,2): s="rLFRluurLFRl"
        elif loc1==(2,0,1): s="bFRBfuubFRBf"
        else: s="lRBLruulRBLr"
        return s
    dir1=cube.rotations[0,0,1]
    dir2=cube.rotations[1,0,2]
    dir3=cube.rotations[2,0,1]
    dir4=cube.rotations[1,0,0]
    d1="0"; d2="0"; d3="0"; d4="0"
    if int(dir1.get_column(1)[0])==1: d1="1"
    if int(dir2.get_column(1)[2])==-1: d2="1"
    if int(dir3.get_column(1)[0])==-1: d3="1"
    if int(dir4.get_column(1)[2])==1: d4="1"
    os=d1+d2+d3+d4
    if os != "0000":
        os += os
        if os[0:4]=="0101": s="BtUrrttuuLuulttuurrTubuu"
        if os[1:5]=="0101": s="LtUbbttuuFuufttuubbTuluu"
        if os[2:6]=="0101": s="FtUllttuuRuurttuullTufuu"
        if os[2:6]=="0101": s="RtUffttuuBuubttuuffTuruu"
        if s!="": return s
        if os[0:2]=="11": s="FRLtUbbttuuFuufttuubbTuluurf"
        if os[1:3]=="11": s="RBFtUllttuuRuurttuullTufuubr"
        if os[2:4]=="11": s="BLRtUffttuuBuubttuuffTuruulb"
        if os[3:5]=="11": s="LFBtUrrttuuLuulttuurrTubuufl"
        return s
    
    print "Solution Found"
    return ""
        
