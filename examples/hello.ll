cmt Hello world

label .ENTRY
    print "Hello World!"
    
    var age 21

    jmp check_driver_eligibility
    cmd_eq TEMP 1 print "You can drive!"
    cmd_eq TEMP 0 print "You can't drive yet :C"

label check_driver_eligibility
    require age
    test_gt_eq age 18