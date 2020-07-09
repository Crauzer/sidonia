use winapi::shared::minwindef::LPVOID;

#[repr(C)]
pub struct RiotIssueOrders {
    /*
        struct __cppobj IssueOrders : IssueOrdersInterface
    {
      order_status_e savedOrderStatus;
      orders_e savedOrderCmd;
      r3dPoint3D savedOrderPos;
      gobjid_t savedOrderObj;
      NavigationPath path;
    };

         */
    issue_orders_interface: RiotIIssueOrders,
}

#[repr(C)]
pub struct RiotNoIssueOrder {
    issue_orders_interface: RiotIIssueOrders,
}

#[repr(C)]
pub struct RiotIIssueOrders {
    vtable: LPVOID,
}
