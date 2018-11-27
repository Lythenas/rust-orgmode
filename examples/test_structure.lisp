(org-data
    nil
    (section
        (:begin 1
            :end 33
            :contents-begin 1
            :contents-end 33
            :post-blank 0
            :parent #0)
        (paragraph
            (:begin 1
                :end 15
                :contents-begin 1
                :contents-end 14
                :post-blank 1
                :post-affiliated 1
                :parent #1)
            #("Some content\n" 0 13 (:parent #2)))
        (keyword
            (:key "ATTR_TEST"
                :value "TEST"
                :begin nil
                :end 33
                :post-blank 0
                :post-affiliated 15
                :parent #1)))
    (headline
        (:raw-value "Test 1"
            :begin 33
            :end 159
            :pre-blank 0
            :hiddenp outline
            :contents-begin 67
            :contents-end 158
            :level 1
            :priority 65
            :tags ("tag1" "tag2")
            :todo-keyword nil
            :todo-type nil
            :post-blank 1
            :footnote-section-p nil
            :archivedp nil
            :commentedp nil
            :quotedp nil
            :KEY1 "VALUE1"
            :CATEGORY "test"
            :title (#("Test 1" 0 6 (:parent #1)))
            :parent #0)
        (section
            (:begin 67
                :end 115
                :contents-begin 67
                :contents-end 114
                :post-blank 1
                :parent #1)
            (property-drawer
                (:begin 67
                    :end 101
                    :hiddenp outline
                    :contents-begin 80
                    :contents-end 94
                    :post-blank 1
                    :post-affiliated 67
                    :parent #2)
                (node-property
                    (:key "KEY1"
                        :value "VALUE1"
                        :begin 80
                        :end 94
                        :post-blank 0
                        :parent #3)))
            (paragraph
                (:begin 101
                    :end 114
                    :contents-begin 101
                    :contents-end 114
                    :post-blank 0
                    :post-affiliated 101
                    :parent #2)
                #("Some content\n" 0 13 (:parent #3))))
        (headline
            (:raw-value "Test 2"
                :begin 115
                :end 158
                :pre-blank 1
                :hiddenp outline
                :contents-begin 147
                :contents-end 158
                :level 2
                :priority nil
                :tags ("tag3")
                :todo-keyword nil
                :todo-type nil
                :post-blank 0
                :footnote-section-p nil
                :archivedp nil
                :commentedp nil
                :quotedp nil
                :CATEGORY "test"
                :title (#("Test 2" 0 6 (:parent #2)))
                :parent #1)
            (headline
                (:raw-value "Test 3"
                    :begin 147
                    :end 158
                    :pre-blank 0
                    :hiddenp nil
                    :contents-begin nil
                    :contents-end nil
                    :level 3
                    :priority nil
                    :tags nil
                    :todo-keyword nil
                    :todo-type nil
                    :post-blank 0
                    :footnote-section-p nil
                    :archivedp nil
                    :commentedp nil
                    :quotedp nil
                    :CATEGORY nil
                    :title (#("Test 3" 0 6 (:parent #3)))
                    :parent #2)))))

