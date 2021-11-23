import Foundation
import ErgoLibC

class Contract {
    internal var pointer: ContractPtr
    
    internal init(withRawPtr ptr: ContractPtr) {
        self.pointer = ptr
    }
    
    init(fromErgoTree: ErgoTree) throws {
        var ptr:  ContractPtr?
        let error = ergo_wallet_contract_new(fromErgoTree.pointer, &ptr)
        try checkError(error)
        self.pointer = ptr!
    }
    
    init(compileFromString: String) throws {
        var ptr: ContractPtr?
        let error = compileFromString.withCString { cs in
            ergo_wallet_contract_compile(cs, &ptr)
        }
        try checkError(error)
        self.pointer = ptr!
    }
    
    init(payToAddress: Address) throws {
        var ptr: ContractPtr?
        let error = ergo_wallet_contract_pay_to_address(payToAddress.pointer, &ptr)
        try checkError(error)
        self.pointer = ptr!
    }
    
    
    func getErgoTree() throws -> ErgoTree {
        var boxIdPtr: ErgoTreePtr?
        let error = ergo_wallet_contract_ergo_tree(self.pointer, &boxIdPtr)
        try checkError(error)
        return ErgoTree(withPtr: boxIdPtr!)
    }
        
    deinit {
        ergo_wallet_contract_delete(self.pointer)
    }
}
