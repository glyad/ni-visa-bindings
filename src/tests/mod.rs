use crate::ffi::*;
const DEVICE_ADDRESS: &'static [u8; 43] = b"USB0::0x0957::0x5407::MY59002371::0::INSTR\0";
const DEVICE_ADDRESS_PTR: *const u8 = DEVICE_ADDRESS.as_ptr();

unsafe fn setup(method_under_test: &str) -> (ViSession, ViSession, ViStatus) {
    print_test_header(method_under_test);
    let mut default_rm: ViSession = 0;

    assert_eq!( 
        viOpenDefaultRM(&mut default_rm),
        VI_SUCCESS.try_into().unwrap(),
        "Failed to open default resource"
    );

    println!("Resource Manager ID is {}", default_rm);
    // Open a resource
    let mut session: ViSession = 0;
    let status_open = viOpen(
        default_rm,
        DEVICE_ADDRESS_PTR as *const i8,
        VI_NULL,
        VI_NULL,
        &mut session,
    );

    println!("Session ID is {}", session);

    (default_rm, session, status_open)
}

fn print_test_header(method_under_test: &str) {
    println!("\n=================================================\n");
    println!("\n\tMethod under test is {}\n", method_under_test);
    println!("\n=================================================\n");
}

fn print_test_footer() {
    println!("\n=================================================\n");
}

unsafe fn teardown(default_rm: ViSession, session: ViSession) {
    println!("Closing Session ID is {}", session);
    println!("Closing Resource Manager ID {}", default_rm);
    let status_close_session = viClose(session);
    viClose(session);
    assert_eq!(
      status_close_session as i32, // Cast to i32
      VI_SUCCESS.try_into().unwrap(),
      "Failed to close resource manager"
    );
    let status_close_rm = viClose(default_rm);
    assert_eq!(
      status_close_rm as i32, // Cast to i32
      VI_SUCCESS.try_into().unwrap(),
      "Failed to close resource manager"
    );
    print_test_footer();
}
#[test]
fn test_visa_open_default_rm() {
    println!("test_visa_open_default_rm");

    let mut default_rm: ViSession = 0;

    unsafe {
        let status = viOpenDefaultRM(&mut default_rm);
        assert!(status >= 0, "Failed to open default resource manager");

        // Close the resource manager
        let status_rm_close = viClose(default_rm);
        assert!(status_rm_close >= 0, "Failed to close resource manager");
    }
}

#[test]
fn test_visa_open_close() {
    println!("test_visa_open_close");

    let mut default_rm: ViSession = 0;
    let mut session: ViSession = 0;

    unsafe {
        // Open Resource Manager
        let status_rm = viOpenDefaultRM(&mut default_rm);
        assert!(status_rm >= 0, "Failed to open default resource manager");

        // Open a TCP/IP resource
        let resource_name = DEVICE_ADDRESS;
        let status_open = viOpen(
            default_rm,
            resource_name.as_ptr() as *const i8,
            VI_NULL,
            VI_NULL,
            &mut session,
        );
        assert!(status_open >= 0, "Failed to open session");

        // Close the session
        let status_close = viClose(session);
        assert!(status_close >= 0, "Failed to close session");

        // Close the resource manager
        let status_rm_close = viClose(default_rm);
        assert!(status_rm_close >= 0, "Failed to close resource manager");
    }
}

#[test]
fn test_find_all_resources() {
    print_test_header("test_find_all_resources");

    unsafe {
        // Open the default resource manager
        let mut default_rm: ViSession = 0;
        let status_rm = viOpenDefaultRM(&mut default_rm);
        assert_eq!(
            status_rm as i32,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to open default resource manager"
        );

        // Variables for viFindRsrc
        let mut find_list: ViFindList = 0;
        let mut return_count: ViUInt32 = 0;
        let mut resource_name: [ViChar; 256] = [0; 256]; // Buffer to hold the resource name

        // Search for all resources
        let search_expression = b"?*INSTR\0"; // Wildcard for all instruments
        let status_find = viFindRsrc(
            default_rm,
            search_expression.as_ptr() as *const i8,
            &mut find_list,
            &mut return_count,
            resource_name.as_mut_ptr(),
        );

        assert_eq!(
            status_find as i32,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to find resources with viFindRsrc"
        );

        // Print the first found resource
        let first_resource = std::ffi::CStr::from_ptr(resource_name.as_ptr())
            .to_string_lossy()
            .to_string();

        println!("Found resource: {}", first_resource);

        // Iterate through additional resources (if any)
        for _ in 1..return_count {
            let status_next = viFindNext(find_list, resource_name.as_mut_ptr());
            assert_eq!(
                status_next as i32, // Cast `status_next` to i32
                VI_SUCCESS.try_into().unwrap(),
                "Failed to find next resource with viFindNext"
            );

            let resource_name_str = std::ffi::CStr::from_ptr(resource_name.as_ptr())
                .to_string_lossy()
                .to_string();
            println!("Found resource: {}", resource_name_str);
        }

        // Clean up
        let status_close_list = viClose(find_list);
        assert_eq!(
            status_close_list as i32, // Cast to i32
            VI_SUCCESS.try_into().unwrap(),
            "Failed to close find list"
        );

        let status_close_rm = viClose(default_rm);
        assert_eq!(
            status_close_rm as i32, // Cast to i32
            VI_SUCCESS.try_into().unwrap(),
            "Failed to close resource manager"
        );
    }
    print_test_footer();
}

#[test]
fn test_vi_find_next() {
    println!("test_vi_find_next");
    unsafe {
        let mut default_rm: ViSession = 0;
        assert_eq!(
            viOpenDefaultRM(&mut default_rm),
            VI_SUCCESS.try_into().unwrap()
        );

        let mut find_list: ViFindList = 0;
        let mut return_count: ViUInt32 = 0;
        let mut resource_name: [ViChar; 256] = [0; 256];

        // Find resources
        let status_find = viFindRsrc(
            default_rm,
            b"?*INSTR\0".as_ptr() as *const i8,
            &mut find_list,
            &mut return_count,
            resource_name.as_mut_ptr(),
        );
        assert_eq!(status_find, VI_SUCCESS.try_into().unwrap());

        // Find next resource
        let status_next = viFindNext(find_list, resource_name.as_mut_ptr());
        assert_eq!(
            status_next,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to find next resource"
        );

        // Print the resource name
        let resource = std::ffi::CStr::from_ptr(resource_name.as_ptr());
        println!("Next resource: {}", resource.to_string_lossy());

        // Cleanup
        viClose(default_rm);
    }
}

#[test]
fn test_vi_parse_rsrc() {
    println!("test_vi_parse_rsrc");
    unsafe {
        let mut default_rm: ViSession = 0;
        assert_eq!(
            viOpenDefaultRM(&mut default_rm),
            VI_SUCCESS.try_into().unwrap()
        );

        let mut interface_type: ViUInt16 = 0;
        let mut interface_number: ViUInt16 = 0;

        let status = viParseRsrc(
            default_rm,
            DEVICE_ADDRESS_PTR as *const i8,
            &mut interface_type,
            &mut interface_number,
        );
        assert_eq!(
            status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to parse resource"
        );

        println!(
            "Parsed resource: Interface type = {}, Interface number = {}",
            interface_type, interface_number
        );

        // Cleanup
        viClose(default_rm);
    }
}

#[test]
fn test_vi_parse_rsrc_ex() {
    println!("test_vi_parse_rsrc_ex");
    unsafe {
        let mut default_rm: ViSession = 0;
        assert_eq!(
            viOpenDefaultRM(&mut default_rm),
            VI_SUCCESS.try_into().unwrap()
        );

        let mut interface_type: ViUInt16 = 0;
        let mut interface_number: ViUInt16 = 0;
        let mut resource_class: [ViChar; 256] = [0; 256];
        let mut expanded_resource: [ViChar; 256] = [0; 256];
        let mut alias_if_exists: [ViChar; 256] = [0; 256];

        let status = viParseRsrcEx(
            default_rm,
            DEVICE_ADDRESS_PTR as *const i8,
            &mut interface_type,
            &mut interface_number,
            resource_class.as_mut_ptr(),
            expanded_resource.as_mut_ptr(),
            alias_if_exists.as_mut_ptr(),
        );
        assert_eq!(
            status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to parse resource (extended)"
        );

        println!(
    "Parsed resource (extended): Interface type = {}, Interface number = {}, Resource class = {}, Expanded resource = {}",
    interface_type,
    interface_number,
    std::ffi::CStr::from_ptr(resource_class.as_ptr()).to_string_lossy(),
    std::ffi::CStr::from_ptr(expanded_resource.as_ptr()).to_string_lossy(),
  );

        // Cleanup
        viClose(default_rm);
    }
}

#[test]
fn test_vi_open() {
    println!("test_vi_open");
    unsafe {
        let mut default_rm: ViSession = 0;
        assert_eq!(
            viOpenDefaultRM(&mut default_rm),
            VI_SUCCESS.try_into().unwrap()
        );

        let mut session: ViSession = 0;
        let status = viOpen(
            default_rm,
            DEVICE_ADDRESS_PTR as *const i8,
            VI_NULL,
            VI_NULL,
            &mut session,
        );
        assert_eq!(
            status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to open resource"
        );

        println!("Resource opened successfully: Session = {}", session);

        // Cleanup
        teardown(default_rm, session);
    }
}

#[test]
fn test_vi_close() {
    unsafe {
        let (default_rm, session, status_open) = setup("test_vi_close");
        print_status_description(default_rm, status_open);
        assert_eq!(
            status_open,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to open the session"
        );

        // Close the session
        let status_close = viClose(session);
        assert_eq!(
            status_close,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to close session"
        );

        println!("Session closed successfully");

        // Cleanup
        viClose(default_rm);
    }
}

#[test]
fn test_vi_set_attribute() {
    unsafe {
        let (default_rm, session, status_open) = setup("test_vi_set_attribute");
        assert_eq!(
            status_open,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to open session"
        );

        let status = viSetAttribute(session, VI_ATTR_TMO_VALUE, 5000); // Set timeout to 5000ms
        print_status_description(default_rm, status);
        assert_eq!(
            status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to set attribute"
        );

        teardown(default_rm, session);
    }
}

#[test]
fn test_vi_get_attribute() {
    unsafe {
        let (default_rm, session, _) = setup("test_vi_get_attribute");

        let status = viSetAttribute(session, VI_ATTR_TMO_VALUE, 5000); // Set timeout to 5000ms
        print_status_description(default_rm, status);
        assert_eq!(
            status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to set attribute"
        );

        let mut timeout: ViUInt32 = 0;

        let status = viGetAttribute(
            session,
            VI_ATTR_TMO_VALUE,
            &mut timeout as *mut _ as *mut std::os::raw::c_void,
        );
        print_status_description(default_rm, status);

        assert_eq!(
            status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to get attribute"
        );
        assert_eq!(timeout, 5000, "Expected default timeout of 2000ms");

        println!("Session #: {}", session);

        teardown(default_rm, session);
    }
}

#[test]
fn test_vi_status_desc() {
    unsafe {
        let mut default_rm: ViSession = 0;
        assert_eq!(
            viOpenDefaultRM(&mut default_rm),
            VI_SUCCESS.try_into().unwrap()
        );

        let mut desc: [ViChar; 256] = [0; 256];
        let status = viStatusDesc(
            default_rm,
            VI_SUCCESS.try_into().unwrap(),
            desc.as_mut_ptr(),
        );
        assert_eq!(
            status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to get status description"
        );

        let description = std::ffi::CStr::from_ptr(desc.as_ptr())
            .to_string_lossy()
            .to_string();
        assert_eq!(description, "Operation completed successfully.");

        viClose(default_rm);
    }
}

#[test]
fn test_vi_terminate() {
    unsafe {
      let (default_rm, session, _) = setup("test_vi_terminate");

        let status = viTerminate(session, VI_NULL as ViUInt16, VI_NULL);
        assert_eq!(
            status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to terminate operation"
        );

        teardown(default_rm, session);
    }
}

#[test]
fn test_vi_lock() {
    unsafe {
      let (default_rm, session, _) = setup("test_vi_lock");

        let lock_status = viLock(
            session,
            VI_EXCLUSIVE_LOCK,
            0,
            VI_NULL as ViConstKeyId,
            VI_NULL as *mut ViChar,
        );
        print_status_description(default_rm, lock_status);
        assert_eq!(
            lock_status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to acquire lock"
        );

        let unlock_status = viUnlock(session);
        print_status_description(default_rm, unlock_status);
        assert_eq!(
            unlock_status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to release lock"
        );

        teardown(default_rm, session);
    }
}
#[test]
fn test_vi_enable_event() {
    unsafe {
      let (default_rm, session, _) = setup("test_vi_enable_event");

        let enable_event_status = viEnableEvent(
            session,
            VI_EVENT_IO_COMPLETION,
            VI_QUEUE.try_into().unwrap(),
            VI_NULL,
        );
        assert_eq!(
            enable_event_status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to enable event"
        );

        teardown(default_rm, session);
    }
}

#[test]
fn test_vi_disable_event() {
    unsafe {
      let (default_rm, session, _) = setup("test_vi_disable_event");

        let enable_event_status = viEnableEvent(
            session,
            VI_EVENT_IO_COMPLETION,
            VI_QUEUE.try_into().unwrap(),
            VI_NULL,
        );
        assert_eq!(
            enable_event_status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to enable event"
        );

        let disable_event_status =
            viDisableEvent(session, VI_EVENT_IO_COMPLETION, VI_QUEUE as ViUInt16);
        assert_eq!(
            disable_event_status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to disable event"
        );

        teardown(default_rm, session);
    }
}

#[test]
fn test_vi_discard_events() {
    unsafe {
      let (default_rm, session, _) = setup("test_vi_discard_events");

        let status = viDiscardEvents(session, VI_ALL_ENABLED_EVENTS, VI_ALL_MECH as ViUInt16);
        print_status_description(default_rm, status);
        assert!(
            status == VI_SUCCESS.try_into().unwrap()
                || status == VI_SUCCESS_QUEUE_EMPTY.try_into().unwrap(),
            "Failed to discard events"
        );

        teardown(default_rm, session);
    }
}

#[test]
fn test_vi_wait_on_event() {
    unsafe {
        let (default_rm, session, _) = setup("test_vi_wait_on_event");

        let mut event_type: ViEventType = 0;
        let mut event_context: ViEvent = 0;

        let status = viWaitOnEvent(
            session,
            VI_EVENT_IO_COMPLETION,
            5000,
            &mut event_type,
            &mut event_context,
        );
        print_status_description(default_rm, status);
        assert_ne!(
            status,
            VI_SUCCESS.try_into().unwrap(),
            "Unexpected success waiting for event"
        );

        teardown(default_rm, session);
    }
}

#[test]
fn test_vi_install_handler() {
    unsafe extern "C" fn handler(
        _vi: ViSession,
        _event_type: ViEventType,
        _event_context: ViEvent,
        _user_handle: ViAddr,
    ) -> i32 {
        println!("Event handler invoked");
        VI_SUCCESS.try_into().unwrap()
    }

    unsafe {
      let (default_rm, session, _) = setup("test_vi_install_handler");

        let status = viInstallHandler(
            session,
            VI_EVENT_IO_COMPLETION,
            Some(handler), // Correctly matches expected function signature
            VI_NULL as *mut std::os::raw::c_void,
        );
        assert_eq!(
            status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to install handler"
        );

        teardown(default_rm, session);
    }
}

#[test]
fn test_vi_read() {
    unsafe {
      let (default_rm, session, _) = setup("test_vi_read");

        let scpi_command = b"*IDN?\n";
        let mut write_count: ViUInt32 = 0;
        let write_status = viWrite(
            session,
            scpi_command.as_ptr() as *const ViByte,
            scpi_command.len() as ViUInt32,
            &mut write_count,
        );
        assert_eq!(
            write_status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to write data"
        );

        let mut buffer: [u8; 256] = [0; 256];
        let mut read_count: ViUInt32 = 0;
        let read_status = viRead(
            session,
            buffer.as_mut_ptr() as *mut ViByte,
            buffer.len() as ViUInt32,
            &mut read_count,
        );
        print_status_description(default_rm, read_status);
        assert!(
            read_status == VI_SUCCESS.try_into().unwrap()
                || read_status == VI_SUCCESS_MAX_CNT.try_into().unwrap(),
            "Failed to read data"
        );

        println!(
            "On SCPI command written: {}Read {} bytes: {:?}.\nText: {}",
            String::from_utf8_lossy(scpi_command),
            read_count,
            &buffer[..read_count as usize],
            String::from_utf8_lossy(&buffer[..read_count as usize])
        );

        teardown(default_rm, session);
    }
}

#[ignore]
#[test]
fn test_vi_read_async() {
    unsafe {
        let (default_rm, session, _) = setup("test_vi_read_async");

        let scpi_command = b"*IDN?\n";
        let mut write_count: ViUInt32 = 0;
        let write_status = viWrite(
            session,
            scpi_command.as_ptr() as *const ViByte,
            scpi_command.len() as ViUInt32,
            &mut write_count,
        );
        assert_eq!(
            write_status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to write data"
        );

        let mut buffer: [u8; 256] = [0; 256];
        let mut job_id: ViJobId = 0;
        let async_read_status = viReadAsync(
            session,
            buffer.as_mut_ptr() as *mut ViByte,
            buffer.len() as ViUInt32,
            &mut job_id,
        );
        print_status_description(default_rm, async_read_status);
        assert_eq!(
            async_read_status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to start async read"
        );

        println!("Async read job started: Job ID = {}", job_id);

        teardown(default_rm, session);
    }
}

#[ignore]
#[test]
fn test_vi_read_to_file() {
    unsafe {
        let (default_rm, session, _) = setup("test_vi_read_to_file");

        let status = viReadToFile(
            session,
            b"output.dat\0".as_ptr() as *const i8,
            1024,
            VI_NULL as ViPUInt32,
        );
        assert_eq!(
            status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to read to file"
        );

        println!("Data successfully written to output.dat");

        teardown(default_rm, session);
    }
}

#[test]
fn test_vi_write() {
    unsafe {
        let (default_rm, session, _) = setup("test_vi_write");

        let scpi_command = b"*IDN?\n";
        let mut write_count: ViUInt32 = 0;
        let write_status = viWrite(
            session,
            scpi_command.as_ptr() as *const ViByte,
            scpi_command.len() as ViUInt32,
            &mut write_count,
        );
        assert_eq!(
            write_status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to write data"
        );

        println!("Wrote {} bytes", write_count);

        teardown(default_rm, session);
    }
}

#[test]
fn test_vi_write_func_squ() {
    unsafe {
        let (default_rm, session, _) = setup("test_vi_write_func_squ");

        let scpi_command = b"OUTPut ON; :FREQuency +20.0E+03; :FUNC SIN; VOLTage:OFFSet 2mV; :FUNCtion:ARBitrary:PTPeak 2";
        let mut write_count: ViUInt32 = 0;
        let write_status = viWrite(
            session,
            scpi_command.as_ptr() as *const ViByte,
            scpi_command.len() as ViUInt32,
            &mut write_count,
        );
        assert_eq!(
            write_status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to write data"
        );
        println!("Wrote {} bytes", write_count);

        let scpi_command2 = b"SYSTem:ERRor?";
        let mut write_count: ViUInt32 = 0;
        let err_write_status = viWrite(
            session,
            scpi_command2.as_ptr() as *const ViByte,
            scpi_command2.len() as ViUInt32,
            &mut write_count,
        );
        assert_eq!(
            err_write_status,
            VI_SUCCESS.try_into().unwrap(),
            "Failed to write data"
        );
        println!("Wrote {} bytes", write_count);

        let mut buffer: [u8; 256] = [0; 256];
        let mut read_count: ViUInt32 = 0;
        let read_status = viRead(
            session,
            buffer.as_mut_ptr() as *mut ViByte,
            buffer.len() as ViUInt32,
            &mut read_count,
        );
        print_status_description(default_rm, read_status);
        assert!(
            read_status == VI_SUCCESS.try_into().unwrap()
                || read_status == VI_SUCCESS_MAX_CNT.try_into().unwrap(),
            "Failed to read data"
        );
        println!(
            "On SCPI command written: {}.\nRead {} bytes: {:?}.\nText: {}",
            String::from_utf8_lossy(scpi_command2),
            read_count,
            &buffer[..read_count as usize],
            String::from_utf8_lossy(&buffer[..read_count as usize])
        );

        teardown(default_rm, session);
    }
}

fn print_status_description(rm: ViSession, status: ViStatus) {
    let mut desc: [ViChar; 256] = [0; 256];
    unsafe {
        viStatusDesc(rm, status, desc.as_mut_ptr());
        let description = std::ffi::CStr::from_ptr(desc.as_ptr())
            .to_string_lossy()
            .to_string();

        println!("Description: {}", description);
    }
}
